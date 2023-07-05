
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::connection::ServerConnection;
use std::io::Result;

pub struct ServerHandler {

    connection: Arc<Mutex<ServerConnection>>,
} 
impl ServerHandler  {
    pub fn new(connection: ServerConnection) -> Self {
        ServerHandler {
            connection: Arc::new(Mutex::new(connection)),
        }
    }
    pub async fn send_move_request(&self, from: (usize, usize), to: (usize, usize)) -> Result<()> {
        // convert coordinates to a message. This depends on your protocol
        let message = format!("move {} {} {} {}", from.0, from.1, to.0, to.1);
        let mut connection = self.connection.lock().await;
        connection.send(&message)?;
        Ok(())
    }
}
