use std::{net::{IpAddr}};
use pnet::packet::{icmp::echo_reply::EchoReplyPacket, ip::IpNextHeaderProtocols};
use pnet_transport::{icmp_packet_iter, transport_channel, TransportChannelType::Layer4};
use pnet_transport::TransportProtocol;
use pnet::packet::{icmp::{IcmpTypes, echo_request::{IcmpCodes}}, util, Packet};
use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use rand::random;


fn main(){
    // let dst_ip: IpAddr = "192.168.43.1".parse().unwrap();
    let dst_ip: IpAddr = "192.168.124.100".parse().unwrap();
    println!("icpm echo request to target ip:{:#?}",dst_ip);

    let proto = Layer4(TransportProtocol::Ipv4(IpNextHeaderProtocols::Icmp));
    let (mut tx,mut rx) =match transport_channel(1024, proto) {
        Ok((tx,rx)) => (tx,rx),
        Err(e) => {
            panic!("An error occurred while reading: {}", e);
        },
    };
    let mut iter = icmp_packet_iter(&mut rx);

    loop {
        let mut icmp_header:[u8;40000] = [0xff;40000];
        let icmp_packet = create_icmp_packet(&mut icmp_header);   
        tx.send_to(icmp_packet, dst_ip).expect("fsd");

        match iter.next() {
            Ok((packet,addr)) => match EchoReplyPacket::new(packet.packet()) {
                Some(_) => {
                    if packet.get_icmp_type() == IcmpTypes::EchoReply {
                        println!(
                            "ICMP EchoReply received from {:?}: {:?}",
                            addr,
                            packet.get_icmp_type(),
                        );
                    } else {
                        println!(
                            "ICMP type other than reply (0) received from {:?}: {:?}",
                            addr,
                            packet.get_icmp_type()
                        );
                    }
                }
                None => {}
            },
            Err(e) => {
                println!("An error occurred while reading: {}", e);
            }
            
        }
    }
}
fn create_icmp_packet<'a>(icmp_header: &'a mut [u8]) -> MutableEchoRequestPacket<'a> {
    let mut icmp_packet = MutableEchoRequestPacket::new(icmp_header).unwrap();
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    icmp_packet.set_icmp_code(IcmpCodes::NoCode);
    icmp_packet.set_identifier(random::<u16>());
    icmp_packet.set_sequence_number(1);
    let checksum = util::checksum(icmp_packet.packet(), 1);
    icmp_packet.set_checksum(checksum);
    icmp_packet
}

    // let args:Vec<String> = env::args().collect();
    // println!("{:?}",args);
    // println!("当前cli parma length is: {}",args.len())
    // let v:Vec<i32> = (1..5).collect();
    // println!("{:?}",v);
    // let args:Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     panic!("参数错误")
    // }