use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    {
        let server_addr = "127.0.0.1:35000";
        println!("Server is binding to address {} ...", server_addr);
        let socket = UdpSocket::bind(server_addr).expect("   Error. Couldn't bind to address");
        let mut total_bytes: u64 = 0;

        // Receives a single datagram message on the socket.
        // If `buf` is too small to hold the message, it will be cut off.
        let mut buf = [0; 66000];
        loop {
            let (amt, src) = socket.recv_from(&mut buf).expect("   Error. Didn't receive data");

            // Redeclare `buf` as slice of the received data and send reverse data back to origin.
            // let buf = &mut buf[..amt];
            total_bytes = total_bytes + (amt as u64);
            println!("   Received {} bytes from {} (total {})", amt, src, total_bytes);
        }
    } // the socket is closed here
}
