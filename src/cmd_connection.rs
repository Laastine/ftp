use std::net::TcpStream;

use connection;

pub fn command(socket: &mut TcpStream, command: String) {
	connection::send_message(socket, command.into_bytes().to_vec());
	let ans = connection::read_message(&socket);
	println!("{:?}", ans);
}