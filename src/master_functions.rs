use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs::File,
};
//function to handle the connection from the slave process
pub fn handle_communication() {
    println!("Communication thread started");
}

pub fn webpage_display() {
    println!("Webpage display thread started");
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
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
                    choice = "404.html"; // redirect to not found
                    break;
                }
            }
            // try to find the file
            let file = std::fs::File::open(format!(".{}", choice));
            let mut page;
            if file.is_err() {
                page = File::open("404.html").unwrap(); // if the file is not found, return the 404 page
            } else {
                if choice == "/" { // implicit index.html call
                    choice = "/index.html";
                }
                page = File::open(format!(".{}", choice)).unwrap();
            }
            // read the file
            let mut contents = String::new();
            page.read_to_string(&mut contents).unwrap();
            // send the response
            let response = format!(
                "HTTP/1.1 200 OK\r, Content-Length: {}\r\n\r, {}", // the response header
                contents.len(), // the length of the response body
                contents // the response body
            );
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            }
        ("POST", _) => {
            // handle the POST request
        }
        _ => {
            // handle the error
        }
    }
}
