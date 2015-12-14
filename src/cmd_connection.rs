use std::net::TcpStream;

use connection;

pub fn command(socket: &mut TcpStream, command: String) {
	connection::send_message(socket, command.into_bytes().to_vec());
	connection::read_message(&socket, &"220".to_string());
}