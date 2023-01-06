use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;

fn scan_port(target: IpAddr, port: u16, timeout: Duration) {
    let socket_address = SocketAddr::new(target.clone(), port);
    
    if let Ok(stream) = TcpStream::connect_timeout(&socket_address, timeout) {
        println!("open {}", port);
    } else {
    }
}

pub fn scan(target: IpAddr) {
    let timeout = Duration::from_secs(5);

    for port in 1..65535 {
        scan_port(target, port, timeout);
    }
}
