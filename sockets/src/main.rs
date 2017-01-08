use std::time::Duration;
use std::thread;

mod net;

fn main() {
    thread::spawn(move || {
        println!("Setting up the server...");
        net::server::listen_for_connections();
    });
    thread::sleep(Duration::from_secs(2));
    net::client::connect();
    thread::sleep(Duration::from_secs(20));
}
