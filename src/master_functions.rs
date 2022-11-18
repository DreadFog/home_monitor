use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs, path::Path,
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

        web_interact(stream);
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