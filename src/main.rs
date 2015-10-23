use std::env::Args;
use std::env;

mod cmd_connection;

fn main() {
    let addr = parse_cmd_args(env::args());
    cmd_connection::connect_server(addr.0, addr.1)
}

fn parse_cmd_args(args: Args) -> (String, String) {
    let args_vec: Vec<_> = args.collect();
    
    let host = match args_vec.iter().nth(args_vec.iter().position(|x| x == "h").unwrap()+1) {
        Some(h) => h,
        None => panic!("No host defined")
    };

    let port = match args_vec.iter().nth(args_vec.iter().position(|x| x == "p").unwrap()+1) {
        Some(p) => p,
        _ => panic!("No port defined")
    };

    (host.to_string(), port.to_string())
}
