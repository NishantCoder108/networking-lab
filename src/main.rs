use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    // 1. Bind our TCP listener to localhost port 8080
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Raw HTTP Server running at http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(mut socket) => {
                // Read incoming bytes from the client (e.g., browser or curl)
                let mut buffer = [0; 1024];
                socket.read(&mut buffer)?;

                // Define our response body
                let body = "Hello from our custom raw Rust HTTP server!\n";

                // Construct a valid HTTP/1.1 response manually, including Content-Length
                let response = format!(
                    "HTTP/1.1 200 OK\r\n\
                    Content-Type: text/plain; charset=utf-8\r\n\
                    Content-Length: {}\r\n\
                    Connection: close\r\n\
                    \r\n\
                    {}",
                    body.len(),
                    body
                );

                // Write the raw bytes back down the TCP stream
                socket.write_all(response.as_bytes())?;
            }
            Err(e) => {
                println!("Failed to establish connection: {}", e);
            }
        }
    }

    Ok(())
}
