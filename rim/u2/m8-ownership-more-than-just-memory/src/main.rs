use std::net::TcpListener;
use std::thread;
use std::time::Duration;

fn open_socket_for_five_seconds() {
    let _lister = TcpListener::bind("127.0.0.1:5000").unwrap();
    thread::sleep(Duration::from_secs(5));
}

fn main() {
    println!("Hello, Sockets");

    open_socket_for_five_seconds();
    println!("Back in main");
    thread::sleep(Duration::from_secs(5));
}
