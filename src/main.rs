use std::env::Args;
use std::env;
use std::net::{SocketAddr, TcpStream};
use std::io;
use std::convert::AsRef;

mod connection;
mod cmd_connection;

fn main() {
    let to_addr = parse_cmd_args(env::args());
    let mut socket = connection::connect(to_addr);
    loop {
        read_cmd_input(&mut socket);
        println!("{:?}", connection::read_message(&socket));
    }
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

fn read_cmd_input(socket: &mut TcpStream) {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    match input.trim().as_ref() {
        "open" => println!("Not implemented"),
        "cd" => cmd_connection::command(socket, "cd".to_string()),
        "help" => cmd_connection::command(socket, "help".to_string()),
        "close" => println!("Not implemented"),
        "active" => println!("Not implemented"),
        "passive" => println!("Not implemented"),
        "get" => println!("Not implemented"),
        "put" => println!("Not implemented"),
        "ls" => cmd_connection::command(socket, "ls".to_string()),
        "ascii" => cmd_connection::command(socket, "ascii".to_string()),
        "binary" => cmd_connection::command(socket, "binary".to_string()),
        "system" => cmd_connection::command(socket, "system".to_string()),
        "status" => cmd_connection::command(socket, "status".to_string()),
        "quit" => cmd_connection::command(socket, "quit".to_string()),
        _ => println!("unknown command"),
    };
}

