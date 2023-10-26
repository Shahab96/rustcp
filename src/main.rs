use rustcp::VirtualNic;

fn main() -> std::io::Result<()> {
    VirtualNic::new("tun0")?.listen()
}
