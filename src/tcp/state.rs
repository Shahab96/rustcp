use std::io;

use etherparse::{Ipv4HeaderSlice, TcpHeaderSlice};

pub enum State {
    Closed,
    Listen,
    SynSent,
    SynRcvd,
    Estab,
    FinWait1,
    FinWait2,
    CloseWait,
    Closing,
    LastAck,
    TimeWait,
}

impl State {
    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        ip_header: Ipv4HeaderSlice<'a>,
        tcp_header: TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<usize> {
        let mut buf = [0u8; 1500];

        match *self {
            State::Closed => Ok(0),
            Self::Listen => {
                if !tcp_header.syn() {
                    // We received a non SYN packet in the listen state.
                    dbg!("Only expected SYN packet.");
                    return Ok(0);
                }

                // Start establishing a connection
                let mut syn_ack = etherparse::TcpHeader::new(
                    tcp_header.destination_port(),
                    tcp_header.source_port(),
                    0,
                    0,
                );

                syn_ack.syn = true;
                syn_ack.ack = true;

                let ip = etherparse::Ipv4Header::new(
                    syn_ack.header_len(),
                    64,
                    etherparse::ip_number::TCP,
                    ip_header.destination_addr().octets(),
                    ip_header.source_addr().octets(),
                );

                // Write the IP and TCP headers to the buffer
                let unwritten = {
                    let mut unwritten = &mut buf[..];
                    ip.write(&mut unwritten);
                    syn_ack.write(&mut unwritten);

                    // How much space remains in the buffer.
                    unwritten.len()
                };

                nic.send(&buf[..unwritten])
            }
            _ => unimplemented!(),
        }
    }
}
