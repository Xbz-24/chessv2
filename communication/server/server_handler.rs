use std::net::TcpStream;
use crate::communication::server::session_manager::SessionManager;
use std::sync::{Arc, Mutex};

pub struct ServerHandler {
    stream: TcpStream,
    session_manager: Arc<Mutex<SessionManager>>,
}

impl ServerHandler {
    pub fn new(stream: TcpStream, session_manager: Arc<Mutex<SessionManager>>) -> Self {
        Self {
            stream,
            session_manager,
        }
    }

    // More methods here...
}
