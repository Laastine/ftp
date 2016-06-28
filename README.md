# FTP client

Hobby project to learn Rust.
Implemented ~according RFC 959.

Tested with Rust 1.9.

## TODO

- [x] Open command connection
- [x] Login
- [x] pwd command
- [x] cd command
- [x] system command
- [x] status command
- [x] port command
- [x] passive command
- [x] Open passive data connection
- [x] ls command
- [x] logout command
- [x] binary command
- [x] ascii command
- [x] status command
- [x] help
- [x] get, file download
- [x] put, file upload
- [ ] Add download and upload speed indicator
- [ ] Resolve hostname
- [ ] Open active data connection
- [ ] Proper error handling

## Build
```
cargo build --release
./target/release/ftp h 127.0.0.1 p 3333
```

### Test against real FTP server

./target/release/ftp h 193.166.3.2 p 21 (ftp.funet.fi)
