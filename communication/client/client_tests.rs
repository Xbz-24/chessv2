#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect() {
        // Setup
        let client = Client::new("127.0.0.1", 8080, "testuser");
        
        // Execution
        let result = client.connect();
        
        // Assertion
        assert!(result.is_ok());
    }

    #[test]
    fn test_disconnect() {
        // Setup
        let client = Client::new("127.0.0.1", 8080, "testuser");
        client.connect().unwrap();

        // Execution
        let result = client.disconnect();

        // Assertion
        assert!(result.is_ok());
    }

    // Continue with other client methods...
}
