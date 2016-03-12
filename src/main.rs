extern crate regex;

use std::env::Args;
use std::env;
use std::net::{TcpStream};
use std::io::{self, Write};
use std::convert::AsRef;

mod connection;

fn main() {
  let host = parse_cmd_arg(&mut env::args(), "h");
  let port = parse_cmd_arg(&mut env::args(), "p");
  let addr = connection::string_to_addr(host, port);
  let mut socket = connection::connect(addr);
  connection::read_message(&socket);
  read_username(&mut socket);
  read_password(&mut socket);

  loop {
    read_cmd_input(&mut socket);
  }
}

fn parse_cmd_arg(args: &mut Args, option: &str) -> String {
  let args_vec: Vec<_> = args.collect();

  match args_vec.iter().nth(args_vec.iter().position(|x| x == option).unwrap()+1) {
    Some(h) => h.to_string(),
    None => panic!("No host defined")
  }
}

fn read_username(socket: &mut TcpStream) {
  let mut input = String::new();
  print!("username: ");
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut input).unwrap();
  let message = "USER ".to_string() + &input;
  connection::send_message(socket, message.into_bytes().to_vec());
  connection::read_message(&socket);
}

fn read_password(socket: &mut TcpStream) {
  let mut input = String::new();
  print!("password: ");
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut input).unwrap();
  let message = "PASS ".to_string() + &input;
  connection::send_message(socket, message.into_bytes().to_vec());
  connection::read_message(&socket);
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
      connection::read_message(&socket);
    },
    "pwd" => {
      connection::send_message(socket, "PWD\r\n".to_string().into_bytes().to_vec());
      connection::read_message(&socket);
    },
    "close" => println!("Not implemented"),
    "active" => println!("Not implemented"),
    "passive" => {
      connection::set_passive(socket);
    },
    "get" => println!("Not implemented"),
    "put" => println!("Not implemented"),
    "ls" => {
      let data_socket = connection::set_passive(socket);
      connection::send_message(socket, "LIST\r\n".to_string().into_bytes().to_vec());
      connection::read_message(&socket);
      connection::recv_unknown(&data_socket);
      connection::read_message(&socket);
    },
    "ascii" => {
      connection::send_message(socket, "TYPE A\r\n".to_string().into_bytes().to_vec());
      connection::read_message(&socket);
    },
    "binary" => {
      connection::send_message(socket, "TYPE I\r\n".to_string().into_bytes().to_vec());
      connection::read_message(&socket);
    },
    "system" => {
      connection::send_message(socket, "SYST\r\n".to_string().into_bytes().to_vec());
      connection::read_message(&socket);
    },
    "status" => {
      connection::send_message(socket, "STAT\r\n".to_string().into_bytes().to_vec());
      connection::read_message(&socket);
    },
    "help" => println!("Not implemented"),
    "logout" => {
      connection::send_message(socket, "LOGOUT\r\n".to_string().into_bytes().to_vec());
      connection::read_message(&socket);
    },
    _ => println!("unknown command"),
  };
}

