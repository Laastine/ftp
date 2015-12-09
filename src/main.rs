use std::env::Args;
use std::env;
use std::net::SocketAddr;
use std::io;
use std::convert::AsRef;

mod connection;

fn main() {
    let to_addr = parse_cmd_args(env::args());
    let mut socket = connection::connect(to_addr);
    loop {
        println!("{:?}",connection::read_message(&socket));
        //read_cmd_input();
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

fn read_cmd_input() {
    println!("read_cmd_input");
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    match input.trim().as_ref() {
        "open" => println!("Not implemented"),
        "cd" => println!("change dir"),
        "help" => println!("Not implemented"),
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
        "quit" => println!("Not implemented"),
        _ => println!("unknown command"),
    };
}

