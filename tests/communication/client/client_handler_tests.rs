#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{TcpListener, TcpStream};
    use std::thread;
    use std::io::prelude::*;

    #[test]
    fn test_client_handler_receives_message() {
        // Start a dummy server in another thread
        let server = TcpListener::bind("127.0.0.1:7878").unwrap();
        let server_handle = thread::spawn(move || {
            for stream in server.incoming() {
                let mut stream = stream.unwrap();

                let response = "Hello, client!";
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
                break; // accept only one connection for this test
            }
        });

        // Sleep for a bit to give the server time to start
        thread::sleep(std::time::Duration::from_millis(100));

        // Connect a ClientHandler to the dummy server
        let server_address = "127.0.0.1:7878".to_string();
        let mut client_handler = ClientHandler::new(server_address);

        // The ClientHandler should receive the message from the server
        let message = client_handler.receive_message();
        assert_eq!(message, "Hello, client!");

        // Clean up the server thread
        server_handle.join().unwrap();
    }
}
