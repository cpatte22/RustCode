use std::io::Write;
use std::net::TcpStream;

fn main() {
    //Connect to the server at the specified address
    match TcpStream::connect("127.0.0.1:8080") {
        Ok(mut stream) => {
            println!("Successfully connected to server");

            let message = "Hello from client!";
            stream.write(message.as_bytes()).expect("Failed to send message");

            println!("Message sent: {}", message);
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
