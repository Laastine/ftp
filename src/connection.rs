
use std::net::{SocketAddr, TcpStream};
use std::io::{Read, Write};

use regex::Regex;

pub fn read_message(mut socket: &TcpStream) -> String {
  let cr = 0x0d;
  let lf = 0x00;
  let mut trimmed_response_vec: Vec<char> ;
  loop {
    let mut line_buffer: Vec<u8> = Vec::new();
    while line_buffer.len() < 2 || (line_buffer[line_buffer.len()-2] != cr && line_buffer[line_buffer.len()-1] != lf) {
      let byte_buffer: &mut [u8] = &mut [0];
      match socket.read(byte_buffer) {
        Ok(_) => {},
        Err(_) => panic!("Error reading response"),
      }
      line_buffer.push(byte_buffer[0]);
    }
    let response = String::from_utf8(line_buffer).unwrap();
    let chars_to_trim: &[char] = &['\r', '\n'];
    println!("{}", response.trim_matches(chars_to_trim).to_string());
    trimmed_response_vec = response.trim_matches(chars_to_trim).chars().collect();

    if trimmed_response_vec[3] == ' ' {
      break;
    }
  }
  trimmed_response_vec.iter().cloned().collect::<String>()
}

pub fn connect(target: SocketAddr) -> TcpStream {
  let socket = match TcpStream::connect(target) {
    Ok(s) => s,
    Err(err) => panic!("Could not bind: {}", err),
  };
  socket
}

pub fn send_message(socket: &mut TcpStream, data: Vec<u8>) {
  match socket.write_all(&data) {
    Ok(res) => res,
    Err(err) => panic!("{:?}", err),
  };
}

pub fn string_to_addr(host: String, port: String) -> SocketAddr {
  let addr = match format!("{}:{}",host, port).parse::<SocketAddr>() {
    Ok(res) => res,
    Err(err) => panic!("Not a valid address {:?}", err)
  };
  addr
}

pub fn set_state(socket: &mut TcpStream, is_active: bool) {
  if is_active == true {
    send_message(socket, "PORT 127.0.0.1,64,05\r\n".to_string().into_bytes().to_vec());
    let res = read_message(&socket);
  } else {
    send_message(socket, "PASV\r\n".to_string().into_bytes().to_vec());
    let res = read_message(&socket);
    let re = Regex::new(r"(?x)
      (?P<code>\d{3})             # code
      [A-Za-z\s]{23}\(
      (?P<ip>(([\d,]{2,4}){4}?))  # server IP
      (?P<foctet>[\d]{1,3}?)      # port first octet
      ,
      (?P<soctet>[\d]{1,3}?)      # port second octet
      \)").unwrap();

    let caps = re.captures(res.as_str()).unwrap();
    let mut server_ip: String = caps.name("ip").unwrap().split(",").map(|x| format!("{}{}", x, ".")).collect();
    server_ip.pop();
    let port = caps.name("foctet").unwrap().parse::<u32>().unwrap() * 256 + caps.name("soctet").unwrap().parse::<u32>().unwrap();
    println!("{}:{}", server_ip, port);
  }
}
