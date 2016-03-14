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
  let mut return_code = 1;

  while return_code > 0 {
    return_code = read_cmd_input(&mut socket);
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

fn read_cmd_input(socket: &mut TcpStream) -> i16 {
  print!("ftp> ");
  io::stdout().flush().unwrap();
  let stdin = io::stdin();
  let mut input = String::new();
  stdin.read_line(&mut input).unwrap();
  let args: Vec<&str> = input.split_whitespace().collect();
  match args[0].as_ref() {
    "cd" => {
      connection::send_message(socket, format!("CWD {}\r\n", args[1]).to_string().into_bytes().to_vec());
      connection::read_message(&socket);
      1
    },
    "pwd" => {
      connection::send_message(socket, "PWD\r\n".to_string().into_bytes().to_vec());
      connection::read_message(&socket);
      1
    },
    "active" => {
      println!("Not implemented");
      1
    },
    "passive" => {
      connection::set_passive(socket);
      1
    },
    "get" => {
      let data_socket = connection::set_passive(socket);
      connection::send_message(socket, format!("RETR {}\r\n", args[1]).to_string().into_bytes().to_vec());
      connection::read_message(&socket);
      connection::write_data_to_file(&data_socket, args[1].to_string());
      connection::read_message(&socket);
      1
    },
    "put" => {
      let mut data_socket = connection::set_passive(socket);
      connection::send_message(socket, format!("STOR {}\r\n", args[1]).to_string().into_bytes().to_vec());
      connection::read_message(&socket);
      connection::send_data_to_server(&mut data_socket, args[1].to_string());
      connection::read_message(&socket);
      1
    },
    "ls" => {
      let data_socket = connection::set_passive(socket);
      connection::send_message(socket, "LIST\r\n".to_string().into_bytes().to_vec());
      connection::read_message(&socket);
      println!("{}", connection::recv_unknown(&data_socket));
      connection::read_message(&socket);
      1
    },
    "ascii" => {
      connection::send_message(socket, "TYPE A\r\n".to_string().into_bytes().to_vec());
      connection::read_message(&socket);
      1
    },
    "binary" => {
      connection::send_message(socket, "TYPE I\r\n".to_string().into_bytes().to_vec());
      connection::read_message(&socket);
      1
    },
    "system" => {
      connection::send_message(socket, "SYST\r\n".to_string().into_bytes().to_vec());
      connection::read_message(&socket);
      1
    },
    "status" => {
      connection::send_message(socket, "STAT\r\n".to_string().into_bytes().to_vec());
      connection::read_message(&socket);
      1
    },
    "help" => {
      connection::print_help_msg()
    },
    "logout" => {
      println!("Shutting down ftp client");
      0
    },
    _ => {
      println!("unknown command");
      1
    },
  }
}

