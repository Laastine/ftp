use std::net::{SocketAddr, TcpStream};
use std::io::{Read, Write};
use std::str::{FromStr, from_utf8};

pub fn read_message(mut socket: &TcpStream, expected_code: &String) {
    let cr = 0x0d;
    let lf = 0x00;
    let mut return_code = expected_code;
    let mut trimmed_response = String::new();

    while return_code.to_string() == expected_code.to_string() {
        let mut line_buffer: Vec<u8> = Vec::new();
        while line_buffer.len() < 2 || (line_buffer[line_buffer.len()-1] != lf && line_buffer[line_buffer.len()-2] != cr) {
            let byte_buffer: &mut [u8] = &mut [0];
            match socket.read(byte_buffer) {
                Ok(_) => {},
                Err(_) => panic!("Error reading response"),
            }
            line_buffer.push(byte_buffer[0]);
        }
        let response = String::from_utf8(line_buffer).unwrap();
        let chars_to_trim: &[char] = &['\r', '\n'];
        trimmed_response = response.trim_matches(chars_to_trim).to_string();
        println!("{:?}", trimmed_response);
        let trimmed_response_vec: Vec<char> = trimmed_response.chars().collect();
        
        if trimmed_response_vec[3] == ' ' {
            break;
        } else {
            let v: Vec<&str> = trimmed_response.splitn(2, ' ').collect();
            let return_code = &v[0].to_string();
        }
    }
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
