use pnet::packet::{icmp::{IcmpTypes, echo_request::{IcmpCodes}}, util, Packet};
use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use rand::random;
fn main() {
    let mut icmp_header:[u8;10] = [0xff;10];
    let mut icmp_packet = MutableEchoRequestPacket::new(&mut icmp_header).unwrap();
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    icmp_packet.set_icmp_code(IcmpCodes::NoCode);
    icmp_packet.set_identifier(random::<u16>());
    icmp_packet.set_sequence_number(1);
    let checksum = util::checksum(icmp_packet.packet(), 1);
    icmp_packet.set_checksum(checksum);

    

    // println!("{:?}",icmp_packet);

    // println!("{:?}",icmp_packet.get_checksum());

}