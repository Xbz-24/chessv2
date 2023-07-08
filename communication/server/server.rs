use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use crate::communication::server::session_manager::SessionManager;
use crate::communication::server::server_handler::ServerHandler;
use crate::communication::server::connection_listener::ConnectionListener;

pub struct Server {
    listener: TcpListener,
    session_manager: Arc<Mutex<SessionManager>>,
}

impl Server {
    pub fn new(addr: &str) -> std::io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        let session_manager = Arc::new(Mutex::new(SessionManager::new()));
        
        Ok(Self {
            listener,
            session_manager,
        })
    }

    pub fn run(&self) {
        let connection_listener = ConnectionListener::new(self.listener.try_clone().unwrap(), self.session_manager.clone());
        connection_listener.start();
    }

    // More methods here...
}
