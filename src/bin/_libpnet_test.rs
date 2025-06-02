use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::{checksum, Ipv4Flags, Ipv4Packet, MutableIpv4Packet};
use pnet::packet::tcp::{ipv4_checksum, MutableTcpPacket, TcpFlags};
use pnet::transport::transport_channel;
use pnet::transport::TransportChannelType::Layer3;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};

pub fn send_reset(
    source: SocketAddrV4,
    destination: SocketAddrV4,
    sequence_number: u32,
    acknowledgement_number: u32,
) -> io::Result<usize> {
    let protocol = Layer3(IpNextHeaderProtocols::Tcp);
    let (mut tx, mut _rx) = match transport_channel(4096, protocol) {
        Ok((tx, rx)) => (tx, rx),
        Err(e) => {
            return Err(e);
        }
    };

    // IP Header 中不包含选项
    const IPV4_HEADER_LEN: usize = 20;
    // TCP Header 中不包含选项，比如 TSVal 等
    const TCP_HEADER_LEN: usize = 20;
    const TOTAL_LENGTH: u16 = (IPV4_HEADER_LEN + TCP_HEADER_LEN) as _;

    let mut packet = [0u8; IPV4_HEADER_LEN + TCP_HEADER_LEN];
    {
        let mut ip_header = MutableIpv4Packet::new(&mut packet[..]).unwrap();
        ip_header.set_version(4);
        ip_header.set_header_length(5);
        ip_header.set_total_length(TOTAL_LENGTH);
        ip_header.set_identification(257u16.to_be());
        ip_header.set_flags(Ipv4Flags::DontFragment);
        ip_header.set_fragment_offset(0);
        ip_header.set_ttl(64);
        ip_header.set_next_level_protocol(IpNextHeaderProtocols::Tcp);
        ip_header.set_source(source.ip().to_owned());
        ip_header.set_destination(destination.ip().to_owned());
        let imm_header = checksum(&ip_header.to_immutable());
        ip_header.set_checksum(imm_header);
    }
    {
        let mut tcp_header = MutableTcpPacket::new(&mut packet[IPV4_HEADER_LEN..]).unwrap();
        tcp_header.set_source(source.port());
        tcp_header.set_destination(destination.port());
        tcp_header.set_sequence(sequence_number);
        tcp_header.set_acknowledgement(acknowledgement_number);
        tcp_header.set_data_offset(5); // 没有任何选项，因此 TCP Header 长度为 5 * 4 字节
        tcp_header.set_flags(TcpFlags::RST);
        tcp_header.set_window(29200);
        let checksum = ipv4_checksum(&tcp_header.to_immutable(), source.ip(), destination.ip());
        tcp_header.set_checksum(checksum);
    }

    match tx.send_to(
        Ipv4Packet::new(&packet[..]).unwrap(),
        IpAddr::V4(destination.ip().to_owned()),
    ) {
        Ok(n) => Ok(n),
        Err(e) => Err(e),
    }
}

fn main() {
    let n = send_reset(
        SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 12345),
        SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 45678),
        783786044,
        2198547145,
    )
    .unwrap();
    println!("send ok[{:?}]", n);
}