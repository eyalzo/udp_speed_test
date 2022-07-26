use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::time::Instant;
use rand::Rng;
use clap::Parser;

/// UDP speed test client, to be used with a dedicated server (same project)
#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// Size of buffer to send repeatedly.
    /// By default OSX has limited the maximum UDP-package to be 9216 bytes.
    /// Alter this value using the following command in the terminal:
    /// sudo sysctl -w net.inet.udp.maxdgram=65535
    #[clap(short, long, value_parser, default_value_t = 5000)]
    buffer_size: usize,
    #[clap(short, long, value_parser, default_value_t = 1000000)]
    total_send: u64,
    #[clap(short, long, value_parser, default_value = "127.0.0.1:35000")]
    server_addr: SocketAddr,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let loop_count = args.total_send / args.buffer_size as u64;

    println!("Client start...");
    // Bind to local address
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).expect("   Error. Client failed to bind (any) address.");
    println!("   Client is bound to local address {}", socket.local_addr().unwrap());

    // Connect, which is not really a connection because it's UDP, so it always succeeds
    socket.connect(args.server_addr).expect("   Error. Failed to connect.");
    println!("   Will try to send to UDP server {}.", args.server_addr);
    println!("      Up to {} bytes in {} loops of {} bytes buffer.", args.total_send, loop_count, args.buffer_size);

    // Prepare buffer
    let mut rnd = rand::thread_rng();
    let mut buf: Vec<u8> = Vec::with_capacity(args.buffer_size);
    for _ in 0..buf.capacity() {
        buf.push(rnd.gen::<u8>());
    }

    let mut total_bytes: u64 = 0;
    let start_time = Instant::now();
    for _ in 0..loop_count {
        socket.send(&buf).expect("   Error. Failed to send message. Please run 'cargo run --bin udp_server' before running this client.");
        total_bytes += buf.len() as u64;
    }
    let duration = start_time.elapsed();
    let speed_mbps: f64 = (total_bytes as f64) * 8.0 / 1000000.0 / duration.as_secs_f64();
    println!("   Completed {} bytes in {:.3?} = {:.3} Mbps", total_bytes, duration, speed_mbps);

    println!("Client stop.");
    Ok(())
}
