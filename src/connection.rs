use std::net::UdpSocket;
use std::net::SocketAddr;
use std::thread;

fn read_message(socket: UdpSocket) -> Vec<u8> {
    let mut buf: [u8; 1] = [0; 1];
    println!("Reading data");
    let result = socket.recv_from(&mut buf);
    thread::sleep_ms(1000);
    drop(socket);
    let mut data;
    match result {
        Ok((amt, src)) => {
            println!("Received data from {}", src);
            data = Vec::from(&buf[0..amt])
        }
        Err(err) => panic!("Read error: {}", err)
    };
    data
}

pub fn send_message(send_addr: SocketAddr, target: SocketAddr, data: Vec<u8>) {
    let socket = socket(send_addr);
    println!("Sending data");
    let result = socket.send_to(&data, target);
    drop(socket);
    match result {
        Ok(amt) => println!("Sent {} bytes", amt),
        Err(err) => panic!("Socket write error: {}", err),
    }
}

pub fn listen(listen_on: SocketAddr) -> thread::JoinHandle<Vec<u8>> {
    let socket = socket(listen_on);
    let handle =  thread::spawn(move || {
        read_message(socket)
    });
    handle
}

pub fn string_to_addr(host: String, port: String) -> SocketAddr {
    let addr = match format!("{}:{}",host, port).parse::<SocketAddr>() {
        Ok(res) => res,
        Err(err) => panic!("Not a valid address")
     };
     addr
}

fn socket(listen_on: SocketAddr) -> UdpSocket {
    let socket = match UdpSocket::bind(listen_on)  {
        Ok(socket) => { println!("Socket bound");
            socket },
        Err(err) => panic!("Could not bind: {}", err),
    };
    socket
}
