use std::net::{SocketAddr, UdpSocket};
use std::time::Instant;
use clap::Parser;

/// UDP speed test server
#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long, value_parser, default_value = "127.0.0.1:35000")]
    server_addr: SocketAddr,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    {
        // Min interval for a print and speed calculation
        const MIN_INTERVAL_MILLIS: usize = 100;
        println!("Server is binding to address {} ...", args.server_addr);
        let socket = UdpSocket::bind(args.server_addr).expect("   Error. Couldn't bind to address");
        let mut total_bytes: u64 = 0;
        let mut interval_bytes: u64 = 0;
        let mut interval_start_time = Instant::now();

        // Receives a single datagram message on the socket.
        // If `buf` is too small to hold the message, it will be cut off.
        // Practically it's impossible to receive more than 67KB
        let mut buf = [0; 100000];
        loop {
            let (amt, _src) = socket.recv_from(&mut buf).expect("   Error. Didn't receive data");
            // Count the interval bytes, and only then see if it is time to calculate and print
            interval_bytes += amt as u64;

            let interval_millis = interval_start_time.elapsed().as_millis() as usize;
            if interval_millis > MIN_INTERVAL_MILLIS {
                interval_start_time = Instant::now();
                total_bytes += interval_bytes;
                let interval_speed_mbps = interval_bytes * 8 / 1000 / interval_millis as u64;
                println!("   Received {} bytes in {} ms = {:.3} Mbps (total {})", interval_bytes,
                         interval_millis, interval_speed_mbps, total_bytes);
                interval_bytes = 0;
            }
        }
    } // the socket is closed here
}
