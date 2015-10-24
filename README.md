# FTP client

This project is for learning to use Rust.
Implemented according RFC 959

## Build
```
cargo build --release
./target/release/ftp h 127.0.0.1 p 3334
```

## Dummy server
```
nc -ul 127.0.0.1 3333
```
