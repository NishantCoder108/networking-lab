use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    // 1. Bind the listener to localhost and port 9000
    let listener = TcpListener::bind("127.0.0.1:9000")?;
    println!("Server listening on 127.0.0.1:9000...");

    println!("{listener:?}");
    // 2. Accept incoming connections (blocking call)
    // The for loop is used here to continuously accept incoming connections.
    // Each iteration of the loop handles a new client connection. This is necessary
    // because a server typically needs to handle multiple clients over time, not just one.
    // The loop will only break if the server is shut down or an unrecoverable error occurs.
    for stream in listener.incoming() {
        println!("{stream:?}");
        match stream {
            Ok(mut socket) => {
                println!("Client connected!");

                // 3. Read raw bytes from the client into a buffer
                // This creates a buffer (an array of 1024 bytes) to temporarily hold data read from the client.
                let mut buffer = [0; 1024];

                let bytes_read = socket.read(&mut buffer)?;

                let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Received message: {}", message.trim());

                // 4. Send a response back to the client
                socket.write_all(b"Hello from Rust TCP Server!\n")?;
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}
