use std::net::{TcpListener, TcpStream};
use std::io::BufReader;
use std::thread;
use std::io::Read;

fn read_messages(stream: &TcpStream) {
    let buf_socket_reader = BufReader::new(stream);
    let mut bytes = Vec::new();
    for byte in buf_socket_reader.bytes() {
        match byte {
            Ok(b) => {
                bytes.push(b); 
            }
            Err(e) => panic!("Byte was not correctly received. Ignoring it. Error: {:?}", e.to_string())
        }
    } 
    println!("server -> {:?}", String::from_utf8(bytes).unwrap());
}

pub fn listen_for_connections() {
    let listener = match TcpListener::bind("127.0.0.1:40002") {
        Ok(l) => l,
        Err(e) => panic!("Server could not be set up to listen. Error: {:?}", e.to_string())
    };
    match listener.accept() {
        Ok(stream) => { 
            let child = thread::spawn(move || {
                read_messages(&stream.0);
            });
            child.join().unwrap();
        }
        Err(e) => panic!("Connection failed. Error: {:?}", e.to_string())
    };
}