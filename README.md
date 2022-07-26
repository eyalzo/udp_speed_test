# Run

There are two binaries. 
You need to first run the server, and only then the client. 
The server will run forever, while the client exists when the test ends.

```bash
$ cargo run --bin udp_server
$ cargo run --bin udp_client
```

# Results

The following tests were performed with OSX locally on a laptop:
MacBook Pro (16-inch, 2019), 8 core 2.4 GHz, 32 GB memory.

Single client and single server. For example:
```bash
$ cargo run --release --bin udp_server
$ cargo run --release --bin udp_client -- -t 1000000000 -b 65500
```

| Buffer | Speed<br>1 client | Speed<br>2 clients |
|--------|-------------------|--------------------|
|   500  |  1.1 Gbps | |
|  1000  |  2.4 Gbps | |
|  1500  |  3.5 Gbps | |
|  2000  |  4.7 Gbps | |
|  2500  |  5.9 Gbps | |
|  3000  |  6.9 Gbps | |
|  3500  |  8.0 Gbps | |
|  4000  |  9.1 Gbps | |
|  5000  | 10.8 Gbps | |
| 10000  | 19.4 Gbps | 15.8 Gbps |
| 20000  | 32.8 Gbps | 28.5 Gbps |
| 30000  | 42.9 Gbps | |
| 40000  | 50.0 Gbps | |
| 50000  | 55.6 Gbps | |
| 60000  | 60.4 Gbps | |
| 65500  | 63.5 Gbps | |

# Client

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
    -s, --server-addr <SERVER_ADDR>    [default: 127.0.0.1:35000]
    -t, --total-send <TOTAL_SEND>      [default: 1000000]
    -V, --version                      Print version information
```

## Message (buffer) size

By default, OSX has limited the maximum UDP-package to be 9216 bytes.
Alter this value using the following command in the terminal:

```bash
$ sudo sysctl -w net.inet.udp.maxdgram=65535 
net.inet.udp.maxdgram: 9216 -> 65535
```

# Technical notes

## Multiple clients vs. a single server

When running multiple clients, they may exhaust the buffers and get this error when calling send():
`code: 55, kind: Uncategorized, message: "No buffer space available"`.

To look at the UDP buffers used by the server, which is an indication for full send buffers, run the following (in OSX):

```bash
$ ./target/release/udp_client -t 100000000000 -b 5000 & ./target/release/udp_client -t 100000000000 -b 5000
$ % lsof -P -N -s -i4udp@localhost:35000
COMMAND     PID      USER   FD   TYPE             DEVICE   SIZE NODE NAME
udp_serve 51465 eyalzohar    3u  IPv4 0x65bab850e63356fd 779960  UDP localhost:35000
udp_clien 55754 eyalzohar    3u  IPv4 0x65bab850e633602d      0  UDP localhost:60444->localhost:35000
udp_clien 55755 eyalzohar    3u  IPv4 0x65bab850e3983d1d      0  UDP localhost:61102->localhost:35000
```
