use super::super::super::communication::server::server_handler::ServerHandler;
use super::super::super::communication::client::client::Client;

#[cfg(test)]
mod server_handler_tests {
    use super::*;

    #[test]
    fn test_new_server_handler() {
        let handler = ServerHandler::new();
        assert!(handler.clients.is_empty());
    }

    #[test]
    fn test_add_client() {
        let mut handler = ServerHandler::new();
        let client = Client::new("127.0.0.1", 8000);
        handler.add_client(client);
        assert_eq!(handler.clients.len(), 1);
    }

    #[test]
    fn test_remove_client() {
        let mut handler = ServerHandler::new();
        let client = Client::new("127.0.0.1", 8000);
        handler.add_client(client.clone());
        handler.remove_client(&client);
        assert!(handler.clients.is_empty());
    }

    #[test]
    fn test_broadcast_message() {
        let mut handler = ServerHandler::new();
        let client1 = Client::new("127.0.0.1", 8000);
        let client2 = Client::new("127.0.0.1", 8001);
        handler.add_client(client1);
        handler.add_client(client2);
        handler.broadcast_message("Test message".to_string());
        // Assume that clients have a "last_message_received" property
        assert_eq!(handler.clients[0].last_message_received, "Test message");
        assert_eq!(handler.clients[1].last_message_received, "Test message");
    }
    
    // Continue with other tests for remaining ServerHandler methods
}
