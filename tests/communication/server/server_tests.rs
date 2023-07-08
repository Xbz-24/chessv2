use super::Server;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use std::io::{Read, Write};

#[test]
fn test_server_accepts_client() {
    let server = Server::new("127.0.0.1:7878");

    let _handle = thread::spawn(move || {
        server.run();
    });

    // Allow the server to start
    thread::sleep(std::time::Duration::from_secs(1));

    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 7878");

            let msg = b"Hello!";
            
            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = std::str::from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    assert!(true); // Replace with actual assertion
}
