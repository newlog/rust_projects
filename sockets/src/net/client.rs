use std::io::BufWriter;
use std::io::Write;
use std::io::Error;
use std::net::TcpStream;    

pub struct TCPClient {
    pub tcp_stream: TcpStream
}

impl TCPClient {

    pub fn new(host: &str, port: &str) -> Result<TCPClient, Error> {
        match TCPClient::connect(host, port) {
            Ok(tcp_stream) => {
                Ok(
                    TCPClient {tcp_stream: tcp_stream}
                  )
            },
            Err(e) => Result::Err(e)
        }
    }

    pub fn send_message(&self, msg: &str) {
        println!("client -> Sending message: {}", msg);
        let mut buf_socket_writer = BufWriter::new(&self.tcp_stream);
        match buf_socket_writer.write(msg.as_bytes()) {
            Ok(_) => {},
            Err(e) => println!("Message could not be sent to server. Error: {:?}", e.to_string()),
        }
    }

    fn connect(host: &str, port: &str) -> Result<TcpStream, Error> {
        let uri = format!("{}:{}", host, port);
        match TcpStream::connect(uri.as_str()) {
            Ok(tcp_stream) => Ok(tcp_stream),
            Err(e) => Result::Err(e),
        } 
    }
}