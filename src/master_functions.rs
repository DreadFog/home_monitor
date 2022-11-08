use std::io::Read;

//function to handle the connection from the slave process
pub fn handle_connection(mut stream: std::net::TcpStream) {
    // Read the stream
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // Convert the stream to a string
    let contents = String::from_utf8_lossy(&buffer[..]);
    // Print the stream
    println!("Packet received: {}", contents);
}