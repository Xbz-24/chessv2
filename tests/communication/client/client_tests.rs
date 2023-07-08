#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    #[test]
    fn test_client_new() {
        let client = Client::new("127.0.0.1:8000".to_string(), "testUser".to_string());
        assert_eq!(client.server_ip, "127.0.0.1:8000");
        assert_eq!(client.username, "testUser");
    }

    #[test]
    fn test_client_connect() {
        // Start a dummy server to connect to
        let listener = TcpListener::bind("127.0.0.1:8001").unwrap();
        thread::spawn(move || {
            for stream in listener.incoming() {
                let _stream = stream.unwrap();
            }
        });

        let mut client = Client::new("127.0.0.1:8001".to_string(), "testUser".to_string());
        assert!(client.connect().is_ok());
    }

    #[test]
    fn test_client_disconnect() {
        // Start a dummy server to connect to
        let listener = TcpListener::bind("127.0.0.1:8002").unwrap();
        thread::spawn(move || {
            for stream in listener.incoming() {
                let _stream = stream.unwrap();
            }
        });

        let mut client = Client::new("127.0.0.1:8002".to_string(), "testUser".to_string());
        client.connect().unwrap();
        assert!(client.disconnect().is_ok());
    }

    // More tests go here...
}
