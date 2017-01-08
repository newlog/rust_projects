use std::net::TcpStream;
use std::io::BufWriter;
use std::io::Write;


fn send_messages(stream: &TcpStream) {
    println!("client -> Sending messages");
    let mut buf_socket_writer = BufWriter::new(stream);
    match buf_socket_writer.write(b"Hello!") {
        Ok(_) => {},
        Err(e) => println!("Message could not be sent to server. Error: {:?}", e.to_string()),
    }
}

pub fn connect() {
    match TcpStream::connect("127.0.0.1:40002") {
        Ok(stream) => send_messages(&stream),
        Err(e) => println!("client -> Could not connect to server. Error: {:?}", e.to_string()),
    } 
}