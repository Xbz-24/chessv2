#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    #[test]
    fn test_successful_connection() {
        // Start a server in a new thread
        let handle = thread::spawn(|| {
            let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
            let mut connection_listener = ConnectionListener::new(listener);
            connection_listener.listen_for_connections();
        });

        // Give the server time to start
        thread::sleep(std::time::Duration::from_secs(1));

        // Start a client and connect to the server
        let stream = TcpStream::connect("127.0.0.1:7878");

        // Ensure the connection was successful
        assert!(stream.is_ok());

        // Clean up the server thread
        handle.join().unwrap();
    }

    #[test]
    fn test_failed_connection() {
        // Try to start a client and connect to a non-existing server
        let stream = TcpStream::connect("127.0.0.1:7879");

        // Ensure the connection was unsuccessful
        assert!(stream.is_err());
    }

    // TODO: Add more tests for various scenarios and edge cases
}
