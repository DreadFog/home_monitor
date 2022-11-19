use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs, path::Path, thread,
};
use crate::config::Config;
use std::sync::{Arc, Mutex};
// the master listens on the local port 19999 for slaves hellos
const MASTER_PORT: &str = "19999";
// the slaves listen for update requests on the port 19998
const SLAVE_PORT: &str = "19998";
//function to handle the connection from the slave process
pub fn handle_communication() {
    println!("Communication thread started");
    // slaves list 
    let slaves_list = Arc::new(Mutex::new(Vec::new()));
    // create a first thread that listens for hellos from the slaves
    let hello_thread = thread::spawn(|| {
        handle_hellos(slaves_list);
    });
    // create a second thread that periodically sends update requests to the slaves
    let update_thread = thread::spawn(|| {
        // sleep for 10 seconds
        thread::sleep(std::time::Duration::from_secs(10));
        println!("Update Query");
        send_update(slaves_list);
    });
    // keep the spawned threads alive
    hello_thread.join().unwrap();
    update_thread.join().unwrap();
}
// function to handle the slave hello messages
fn handle_hellos( slaves_list : Arc<Mutex<Vec<Config>>>) {
    let listener = TcpListener::bind(format!("localhost:{}", MASTER_PORT)).unwrap();
    println!("Listening on port {}", MASTER_PORT);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                // clone the slaves list mutex to pass it to the thread
                let slaves_list = Arc::clone(&slaves_list);
                thread::spawn(move || {
                    // add the new config to the slaves list
                    let new_config = handle_client(stream);
                    let mut slaves_list = slaves_list.lock().unwrap();
                    slaves_list.push(new_config);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
// function to handle the client connection
fn handle_client(stream: TcpStream) -> Config {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    // deserialize the config
    let config: Config = serde_json::from_str(&buffer).unwrap();
    // print the config
    println!("Config: {:?}", config);
    // close the connection
    drop(stream);
    // return the config
    config
}

// function to send update requests to the slaves
fn send_update(slaves_list: Arc<Mutex<Vec<Config>>>) {
    // clone the slaves list mutex to pass it to the thread
    let slaves_list = Arc::clone(&slaves_list);
    // get the slaves list
    let slaves_list = slaves_list.lock().unwrap();
    // iterate over the slaves list
    for slave in slaves_list.iter() {
        // create a new thread for each slave
        thread::spawn(move || {
            // create a new connection to the slave
            let mut stream = TcpStream::connect(format!("{}:{}",slave.get_private_ip(), SLAVE_PORT)).unwrap();
            // send the update request
            stream.write("update".as_bytes()).unwrap();
            stream.flush().unwrap();
            // read the response
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut buffer = String::new();
            reader.read_line(&mut buffer).unwrap();
            // deserialize the config
            let config: Config = serde_json::from_str(&buffer).unwrap();
            // print the config
            println!("Config: {:?}", config);
            // close the connection
            drop(stream);
        });
    }

}

pub fn webpage_display() {
    println!("Webpage display thread started");
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || {
            web_interact(stream);
        });
    }
}

fn web_interact(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);
    let mut first_line = http_request[0].split_whitespace();
    // Get the method
    let method = first_line.next().unwrap();
    // Get the path
    let path = first_line.next().unwrap();
    match (method, path) {
        ("GET", _) => {
            // sanitize the choice
            let illegal_choices = &["../", "etc", "home", "root", "var", "id_rsa"];
            let mut choice = path;
            for illegal_choice in illegal_choices {
                if choice.contains(illegal_choice) {
                    choice = "/404.html"; // redirect to not found
                    break;
                } 
            }
            if choice == "/" {
                choice = "/index.html";
            }  
            // try to find the file
            if !Path::new(&format!(".{}", choice)).exists() {
                choice = "/404.html"; // if the file is not found, return the 404 page
            }
            // headers
            let data_type = get_content_type(choice
                .split(".")
                .last(). // get the extension
                unwrap_or_else(|| "text/plain")); // if no extension, return text/plain

            let content_type = format!("Content-Type: {}", data_type);
            let headers_img = [
                "HTTP/1.1 200 OK",
                content_type.as_str(),
                "\r\n"
            ];
            let contents = fs::read(format!(".{}", choice)).unwrap();
            // write the headers
            stream.write(&headers_img.join("\r\n").as_bytes()).unwrap();
            // write the file
            stream.write(&contents).unwrap();
            stream.flush().unwrap();
            }
        ("POST", _) => {
            // handle the POST request
            stream.write("HTTP/1.1 501 NOT IMPLEMENTED\r\n\r\n".as_bytes()).unwrap();
        }
        _ => {
            // handle the error
            stream.write("HTTP/1.1 403 FORBIDDEN\r\n\r\n".as_bytes()).unwrap();
        }
    }
}
fn get_content_type(choice: &str) -> String {
    let types = [format!("image/{}", choice),
                              format!("text/{}", choice),
                              format!("application/{}", choice)];
    match choice {
        "png" | "jpg" | "jpeg" | "gif" => types[0].as_str(),
        "html"| "css" => types[1].as_str(),
        "json"| "xml" | "pdf" | "zip" | "rtf" | "wasm" => types[2].as_str(),
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "txt" => "text/plain",
        "js" => "application/javascript",
        "rar" => "application/x-rar-compressed",
        "7z" => "application/x-7z-compressed",
        "mp3" => "audio/mpeg",
        "mp4" => "video/mp4",
        "wav" => "audio/wav",
        "avi" => "video/x-msvideo",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "ppt" => "application/vnd.ms-powerpoint",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        _ => "text/plain",
    }.to_string()
}

// function to request an update from the slave whose ip is passed as argument
pub fn request_update(ip: &str) -> Option<Config> {
    let stream_wrapped = TcpStream::connect(format!("{}:{}", ip, SLAVE_PORT));
    if stream_wrapped.is_err() {
        eprintln!("Error: {}", stream_wrapped.err().unwrap());
        return None; // no update
    }
    let mut stream = stream_wrapped.unwrap();
    // send the request
    stream.write("update".as_bytes()).unwrap();
    // read the response
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    // deserialize the config
    let config: Config = serde_json::from_str(&buffer).unwrap();
    // print the config
    println!("Config: {:?}", config);
    // close the connection
    drop(stream);
    Some(config)
}
    