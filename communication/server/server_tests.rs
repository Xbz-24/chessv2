#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accept_client() {
        // Setup
        let server = Server::new(8080);
        
        // Execution
        let result = server.accept_client();
        
        // Assertion
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_client() {
        // Setup
        let server = Server::new(8080);
        let client = Client::new("127.0.0.1", 8080, "testuser");
        server.accept_client().unwrap();

        // Execution
        let result = server.handle_client(client);

        // Assertion
        assert!(result.is_ok());
    }

    // Continue with other server methods...
}
