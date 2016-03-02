use std::env::Args;
use std::env;
use std::net::{SocketAddr, TcpStream};
use std::io::{self, Write};
use std::convert::AsRef;

mod connection;

fn main() {
    let to_addr = parse_cmd_args(env::args());
    let mut socket = connection::connect(to_addr);
    connection::read_message(&socket, &"220".to_string());
    read_username(&mut socket);
    read_password(&mut socket);

    loop {
        read_cmd_input(&mut socket);
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

fn read_username(socket: &mut TcpStream) {
    let mut input = String::new();
    print!("username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let message = "USER ".to_string() + &input;
    connection::send_message(socket, message.into_bytes().to_vec());
    connection::read_message(&socket, &"331".to_string());
}

fn read_password(socket: &mut TcpStream) {
    let mut input = String::new();
    print!("password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let message = "PASS ".to_string() + &input;
    connection::send_message(socket, message.into_bytes().to_vec());
    connection::read_message(&socket, &"230".to_string());
}

fn read_cmd_input(socket: &mut TcpStream) {
    print!("ftp> ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let args: Vec<&str> = input.split_whitespace().collect();
    match args[0].as_ref() {
        "open" => println!("Not implemented"),
        "cd" => {
            connection::send_message(socket, format!("CWD {}\r\n", args[1]).to_string().into_bytes().to_vec());
            connection::read_message(&socket, &"250".to_string());
        },
        "pwd" => {
            connection::send_message(socket, "PWD\r\n".to_string().into_bytes().to_vec());
            connection::read_message(&socket, &"257".to_string());
        },
        "close" => println!("Not implemented"),
        "active" => println!("Not implemented"),
        "passive" => println!("Not implemented"),
        "get" => println!("Not implemented"),
        "put" => println!("Not implemented"),
        "ls" => println!("Not implemented"),
        "ascii" => println!("Not implemented"),
        "binary" => println!("Not implemented"),
        "system" => println!("Not implemented"),
        "status" => println!("Not implemented"),
        "help" => println!("Not implemented"),
        "quit" => println!("Not implemented"),
        _ => println!("unknown command"),
    };
}

