use std::io::{self, Write};
use crate::connection::ServerConnection;

pub struct GameInterface {

}
impl GameInterface {
    pub fn new() -> Self {
        GameInterface {

        }
    }
    pub fn start(&self, mut connection: ServerConnection) {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            // send input to server
            connection.send(&input).unwrap();
            // receive response from server
            let response = connection.receive().unwrap();
            // print server response
            println!("{}", response);
        }
    }
    pub fn update_board(&self, _from: (usize, usize), _to: (usize, usize)) {
        
    }
}
