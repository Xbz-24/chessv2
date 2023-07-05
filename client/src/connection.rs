use std::net::TcpStream;
use std::io::{Read, Write, Result};

pub struct ServerConnection {
    stream: TcpStream,
}

impl ServerConnection {
    pub fn new(address: &str) -> Result<ServerConnection> {
        let stream = TcpStream::connect(address)?;
        Ok(ServerConnection { stream })
    }
    pub fn send(&mut self, message: &str) -> Result<()> {
        self.stream.write_all(message.as_bytes())
    }
    pub fn receive(&mut self) -> Result<String> {
        let mut buffer = [0; 512];
        self.stream.read(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }
}
