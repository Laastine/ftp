
use std::net::{SocketAddr, TcpStream, TcpListener};
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

pub fn recv_unknown(mut data_socket: &TcpStream) {
  let mut line_buffer: Vec<u8> = Vec::new();
  while line_buffer.len() < 1 || (line_buffer[line_buffer.len()-1] != 0) {
    let byte_buffer: &mut [u8] = &mut [0];
    match data_socket.read(byte_buffer) {
      Ok(_) => {},
      Err(_) => panic!("Error reading response"),
    }
    line_buffer.push(byte_buffer[0]);
  }
  let response = String::from_utf8(line_buffer).unwrap();
  let chars_to_trim: &[char] = &['\r', '\n'];
  println!("{}", response.trim_matches(chars_to_trim).to_string());  
}

pub fn connect(target: SocketAddr) -> TcpStream {
  let socket = match TcpStream::connect(target) {
    Ok(s) => s,
    Err(err) => panic!("Couldn't bind: {}", err),
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

pub fn set_passive(socket: &mut TcpStream) -> TcpStream {
  send_message(socket, "PASV\r\n".to_string().into_bytes().to_vec());
  let res = read_message(&socket);
  let re = Regex::new(r"(?x)
    (?P<code>\d{3})             # code
    [A-Za-z\s]{23}\(
    (?P<ip1>([\d,]{1,3})?)  # server IP
    ,
    (?P<ip2>([\d]{1,3})?)  # server IP
    ,
    (?P<ip3>([\d]{1,3})?)  # server IP
    ,
    (?P<ip4>([\d]{1,3})?)  # server IP
    ,
    (?P<foctet>[\d]{1,3}?)      # port first octet
    ,
    (?P<soctet>[\d]{1,3}?)      # port second octet
    \)").unwrap();

  let caps = re.captures(res.as_str()).unwrap();
  let server_ip = format!("{}.{}.{}.{}", caps.name("ip1").unwrap(), caps.name("ip2").unwrap(), caps.name("ip3").unwrap(), caps.name("ip4").unwrap()) ;
  let port = caps.name("foctet").unwrap().parse::<u16>().unwrap() * 256 + caps.name("soctet").unwrap().parse::<u16>().unwrap();
  match TcpStream::connect((&*server_ip, port)) {
    Ok(s) => s,
    Err(err) => panic!("Couldn't bind data connection: {}", err),
  }
}

#[allow(dead_code)]
pub fn set_active(socket: &mut TcpStream) {
  let data_sock = init_data_socket();
  let port_cmd = format!("PORT 127,0,0,1,{},{}\r\n", (data_sock.1).0, (data_sock.1).1);
  send_message(socket, port_cmd.into_bytes().to_vec());
  let res = read_message(&socket);
  println!("res {:?}", res);
}

#[allow(dead_code)]
fn init_data_socket() -> (TcpListener, (u16, u16)) {
  let listener = TcpListener::bind("127.0.0.1:0").unwrap();
  let local_addr = listener.local_addr().unwrap();
  let port = local_addr.port();
  let firt_octet = port >> 8;
  let second_octet = port & 0xff;
  (listener, (firt_octet, second_octet))
}
