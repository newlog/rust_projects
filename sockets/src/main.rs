use net::client::TCPClient;
use std::time::Duration;
use std::thread;

mod net;

fn main() {
    start_server();
    thread::sleep(Duration::from_secs(2));
    let client = start_client();
    send_messages(&client);
    thread::sleep(Duration::from_secs(20));
}

fn start_server() {
    thread::spawn(move || {
        println!("Setting up the server...");
        net::server::listen_for_connections();
    });
}

fn start_client() -> net::client::TCPClient {
    match TCPClient::connect("127.0.0.1", "40002") {
        Ok(client) => client,
        Err(e) => panic!("client -> Error connecting to server. Error: {}", e.to_string())
    }
}

fn send_messages(client: &net::client::TCPClient) {
    client.send_message("Hi there!");
    client.send_message("How are you?");
}