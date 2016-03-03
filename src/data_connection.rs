use std::net::{TcpListener, TcpStream};

let listener = TcpListener::bind("127.0.0.1:3333").unwrap();

fn send_port_cmd() {

}

fn handle_client(stream: TcpStream) {

}

// accept connections and process them, spawning a new thread for each one
for stream in listener.incoming() {
    match stream {
        Ok(stream) => {
            thread::spawn(move|| {
                handle_client(stream)
            });
        }
        Err(e) => {
        	println!("Error while handling data connection {}", e);
        }
    }
}

drop(listener);
