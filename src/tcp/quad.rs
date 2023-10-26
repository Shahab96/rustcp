use std::{fmt::Debug, net::Ipv4Addr};

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub struct Quad(u32, u32, u16, u16);

impl Debug for Quad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let src = Ipv4Addr::from(self.0);
        let dst = Ipv4Addr::from(self.1);

        f.write_fmt(format_args!("{}:{} -> {}:{}", src, self.2, dst, self.3))
    }
}

impl Quad {
    pub fn new(src: Ipv4Addr, src_port: u16, dst: Ipv4Addr, dst_port: u16) -> Self {
        Self(u32::from(src), u32::from(dst), src_port, dst_port)
    }
}
