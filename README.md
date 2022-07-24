# Run

There are two binaries. 
You need to first run the server, and only then the client. 
The server will run forever, while the client exists when the test ends.

```bash
cargo run --bin udp_server
cargo run --bin udp_client
```