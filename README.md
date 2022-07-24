# Run

There are two binaries. 
You need to first run the server, and only then the client. 
The server will run forever, while the client exists when the test ends.

```bash
$ cargo run --bin udp_server
$ cargo run --bin udp_client
```

## Client

Run the client only after the server is already running.

```bash
udp_speed_test 0.1.0
UDP speed test client, to be used with a dedicated server (same project)

USAGE:
    udp_client [OPTIONS]

OPTIONS:
    -b, --buffer-size <BUFFER_SIZE>    Size of buffer to send repeatedly. By default OSX has limited
                                       the maximum UDP-package to be 9216 bytes. Alter this value
                                       using the following command in the terminal: sudo sysctl -w
                                       net.inet.udp.maxdgram=65535 [default: 5000]
    -h, --help                         Print help information
    -t, --total-send <TOTAL_SEND>      [default: 1000000]
    -V, --version                      Print version information
```

### Message (buffer) size

By default, OSX has limited the maximum UDP-package to be 9216 bytes.
Alter this value using the following command in the terminal:

```bash
$ sudo sysctl -w net.inet.udp.maxdgram=65535 
net.inet.udp.maxdgram: 9216 -> 65535
```
