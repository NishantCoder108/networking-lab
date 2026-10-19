use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // 1. Connect to the server running on localhost port 9000
    let mut stream = TcpStream::connect("127.0.0.1:9000")?;
    println!("Connected to server!");

    // 2. Send raw bytes across the TCP stream
    stream.write_all(b"Hello Server, can you hear me?\n")?;

    // 3. Read the server's reply into a buffer
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;

    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Server responded: {}", response.trim());

    Ok(())
}
