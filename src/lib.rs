mod tcp;

use std::collections::HashMap;
use std::io::Result;

use etherparse::{Ipv4HeaderSlice, TcpHeaderSlice};
use tun_tap::{Iface, Mode};

pub use tcp::connection::Connection;
pub use tcp::quad::Quad;
pub use tcp::state::State;

pub struct VirtualNic {
    nic: Iface,
    connections: HashMap<Quad, Connection>,
}

impl VirtualNic {
    pub fn new(name: &str) -> Result<Self> {
        Ok(Self {
            nic: Iface::new(name, Mode::Tun)?,
            connections: Default::default(),
        })
    }

    pub fn listen(&mut self) -> Result<()> {
        let mut buf = [0u8; 1504];

        loop {
            let nbytes = self.nic.recv(&mut buf[..])?;

            dbg!("Recvd {} bytes on {}", nbytes, self.nic.name());

            // Bytes 2 and 3 contain the Ethernet protocol
            let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);

            if eth_proto != 0x800 {
                // not IPv4
                dbg!("Ignoring non Ipv4 packet.");
                continue;
            }

            let Ok(ipv4_header) = Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) else {
                dbg!("Ignoring weird IPv4 packet");
                continue;
            };

            if ipv4_header.protocol() != 0x06 {
                // not TCP
                dbg!("Ignoring non-TCP packet");
                continue;
            }

            let Ok(tcp_header) =
                TcpHeaderSlice::from_slice(&buf[4 + ipv4_header.slice().len()..nbytes])
            else {
                println!("Ignoring weird TCP packet");
                continue;
            };

            let src_ip = ipv4_header.source_addr();
            let dst_ip = ipv4_header.destination_addr();
            let src_port = tcp_header.source_port();
            let dst_port = tcp_header.destination_port();
            let quad = Quad::new(src_ip, src_port, dst_ip, dst_port);
            let data_index = 4 + ipv4_header.slice().len() + tcp_header.slice().len();
            let size = nbytes - data_index;

            dbg!("{} bytes of data received for quad {}", quad, size);

            _ = self.connections.entry(quad).or_default().on_packet(
                &mut self.nic,
                ipv4_header,
                tcp_header,
                &buf[data_index..nbytes],
            );
        }
    }
}
