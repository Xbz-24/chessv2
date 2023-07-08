use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use crate::communication::server::session_manager::SessionManager;
use crate::communication::server::server_handler::ServerHandler;
pub struct ConnectionListener {
    listener: TcpListener,
    session_manager: Arc<Mutex<SessionManager>>,
}
impl ConnectionListener {
    pub fn new(listener: TcpListener, session_manager: Arc<Mutex<SessionManager>>) -> Self {
        Self {
            listener,
            session_manager,
        }
    }
    pub fn start(&self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            let handler = ServerHandler::new(stream.try_clone().unwrap(), self.session_manager.clone());
            // spawn new thread to handle the connection
            std::thread::spawn(move || {
                handler.handle_connection();
            });
        }
    }

    // More methods here...
}
