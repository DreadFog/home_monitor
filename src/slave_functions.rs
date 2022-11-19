use std::{net::{TcpStream, TcpListener}, io::{Write, Read}, thread};
use crate::config::Config;
// slaves know the ip address of the master
const MASTER_IP: &str = "192.168.1.10"; // debug
// the master uses the local port 19999 to communicate with the slaves
const MASTER_PORT: &str = "19999";
// the slaves listen for update requests on the port 19998
const SLAVE_PORT: &str = "19998";
// function that sends a slave hello message to the master
pub fn send_hello() {
    let mut stream = TcpStream::connect(format!("localhost:{}", MASTER_PORT)).unwrap();
    let hello = message();
    stream.write(hello.as_bytes()).unwrap();
    stream.flush().unwrap();
    // close the connection
    drop(stream);
}
// function that builds the slave initial message
fn message() -> String {
    // Create the config
    let config = Config::new();
    // Serialize the config
    serde_json::to_string(&config).unwrap()
    
}
// function that listens for update requests from the master
pub fn listen_for_update() {
    let listener = TcpListener::bind(format!("localhost:{}", SLAVE_PORT)).unwrap();
    println!("Listening on port {}", SLAVE_PORT);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_master_request(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
// function that handles the master request
fn handle_master_request(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    // if the request contains the update string, send a new config
    if String::from_utf8_lossy(&buffer[..]).contains("update") {
        println!("Update request received");
        let config = Config::new();
        let config = serde_json::to_string(&config).unwrap();
        stream.write(config.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        eprintln!("Invalid request");
    }
    // close the connection
    drop(stream);
}

