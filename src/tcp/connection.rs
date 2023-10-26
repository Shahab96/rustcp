use std::io;

use etherparse::{Ipv4HeaderSlice, TcpHeader, TcpHeaderSlice};

use super::sequence_space::{recv::ReceiveSequenceSpace, send::SendSequenceSpace};
use super::state::State;

pub struct Connection {
    state: State,
    send: SendSequenceSpace,
    recv: ReceiveSequenceSpace,
}

impl Default for Connection {
    fn default() -> Self {
        return Self {
            state: State::Listen,
            send: SendSequenceSpace::default(),
            recv: ReceiveSequenceSpace::default(),
        };
    }
}

impl Connection {
    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        ip_header: Ipv4HeaderSlice<'a>,
        tcp_header: TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<usize> {
        let mut buf = [0u8; 1500];

        match self.state {
            State::Closed => Ok(0),
            State::Listen => {
                if !tcp_header.syn() {
                    // We received a non SYN packet in the listen state.
                    dbg!("Only expected SYN packet.");
                    return Ok(0);
                }

                // Start establishing a connection
                let mut syn_ack = TcpHeader::new(
                    tcp_header.destination_port(),
                    tcp_header.source_port(),
                    0, // Need to figure out a way to generate this
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
