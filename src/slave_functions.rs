use std::{net::TcpStream, io::Write};
use crate::config::Config;
// slaves know the ip address of the master
const MASTER_IP: &str = "192.168.1.16";
// slaves know the port of the master to communicate with
const MASTER_PORT: &str = "9999";
// function that sends a slave hello message to the master
fn send_hello() {
    let mut stream = TcpStream::connect(format!("{}:{}", MASTER_IP, MASTER_PORT)).unwrap();
    let hello = "Hello from slave";
    stream.write(hello.as_bytes()).unwrap();
    stream.flush().unwrap();
    // close the connection
    drop(stream);
}
// function that builds theslave initial message
fn build_initial_message() -> Config {
    // create the config
    let config = Config::new();
    config // to serialize
    
}