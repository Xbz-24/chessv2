use crate::communication::client::output_handler::OutputHandler;
use crate::communication::message::Message;
use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::thread;

#[test]
fn test_send_message() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();

        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).unwrap();
        let message_str = std::str::from_utf8(&buffer[0..bytes_read]).unwrap();

        assert_eq!(message_str, "Test message\n");
    });

    let stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    let output_handler = OutputHandler::new(stream);

    output_handler.send_message(Message::new("Test message")).unwrap();

    handle.join().unwrap();
}
