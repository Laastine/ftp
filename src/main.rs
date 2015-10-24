use std::env::Args;
use std::env;
use std::net::SocketAddr;

mod connection;

fn main() {
    let from_addr = parse_cmd_args(env::args());
    let to_addr = connection::string_to_addr("127.0.0.1".to_string(), "3333".to_string());
    connection::send_message(from_addr, to_addr, b"hello".to_vec());
}

fn parse_cmd_args(args: Args) -> SocketAddr {
    let args_vec: Vec<_> = args.collect();
    
    let host = match args_vec.iter().nth(args_vec.iter().position(|x| x == "h").unwrap()+1) {
        Some(h) => h,
        None => panic!("No host defined")
    };

    let port = match args_vec.iter().nth(args_vec.iter().position(|x| x == "p").unwrap()+1) {
        Some(p) => p,
        _ => panic!("No port defined")
    };

    connection::string_to_addr(host.to_string(), port.to_string())
}
