# FTP client

Hobby project to learn Rust.
Implemented according RFC 959.

## TODO

- [x] Basic TCP command connection
- [x] Login
- [ ] ls command
- [ ] status command
- [ ] cd command
- [ ] help command
- [ ] active command
- [ ] passive command
- [ ] binary command
- [ ] ascii command
- [ ] system command
- [ ] status command
- [ ] quit command
- [ ] Error handling
- [ ] FTP server domain resolve
- [ ] get/put UDP transaction

## Build
```
cargo build --release
./target/release/ftp h 127.0.0.1 p 3333
```

## Dummy server
```
nc -l 127.0.0.1 3333
```

## Test against real FTP server

./target/release/ftp h 193.166.3.2 p 21 (ftp.funet.fi)
