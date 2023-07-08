use super::client_handler::ClientHandler;
use super::input_handler::InputHandler;
use super::output_handler::OutputHandler;

pub struct Client {
    handler: ClientHandler,
    input: InputHandler,
    output: OutputHandler,
}

impl Client {
    pub fn new(handler: ClientHandler, input: InputHandler, output: OutputHandler) -> Self {
        Client {
            handler,
            input,
            output,
        }
    }

    pub fn connect(&self, server_address: &str) -> std::io::Result<()> {
        self.handler.connect(server_address)
    }

    pub fn disconnect(&self) -> std::io::Result<()> {
        self.handler.disconnect()
    }

    // Other methods...
}
