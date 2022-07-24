use std::net::{Ipv4Addr, UdpSocket};
use std::time::Instant;
use rand::Rng;

fn main() -> std::io::Result<()> {
    let server_addr = "127.0.0.1:35000";
    // By default OSX has limited the maximum UDP-package to be 9216 bytes.
    // Alter this value using the following command in the terminal:
    // sudo sysctl -w net.inet.udp.maxdgram=65535
    const BUFF_SIZE: usize = 5000;
    const TOTAL_TO_SEND: usize = 1000 * 1000 * 1000; // 100 MB
    let loop_count = TOTAL_TO_SEND / BUFF_SIZE;

    println!("Client start...");
    // Bind to local address
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).expect("   Error. Client failed to bind (any) address.");
    println!("   Client is bound to local address {}", socket.local_addr().unwrap());

    // Connect, which is not really a connection because it's UDP, so it always succeeds
    socket.connect(server_addr).expect("   Error. Failed to connect.");
    println!("   Will try to send to UDP server {}.", server_addr);

    // Prepare buffer
    let mut rnd = rand::thread_rng();
    let mut buf: Vec<u8> = Vec::with_capacity(BUFF_SIZE);
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
