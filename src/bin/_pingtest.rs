use std::{net::{IpAddr}, time::{Instant, Duration}, sync::{Arc, RwLock}, env};
use pnet::packet::{ip::{IpNextHeaderProtocols,}, icmp::{IcmpTypes, echo_request::{IcmpCodes, MutableEchoRequestPacket}, echo_reply::EchoReplyPacket}, util, Packet};
use pnet_transport::{transport_channel, TransportProtocol};
use pnet_transport::TransportChannelType::Layer4;
use pnet_transport::{icmp_packet_iter};
use rand::random;

const ICMP_SIZE: usize = 64;
fn main(){

    let args:Vec<String> = env::args().collect();
    if (args.len() < 2) {
        panic!("Usage: icmp-demo target_ip");
    }

    let target_ip:IpAddr = args[1].parse().unwrap();
    println!("icmp echo request to target ip:{:#}",target_ip);

    // let protocol = Layer4(TransportProtocol::Ipv4(IpNextHeaderProtocol(1));
    let protocol = Layer4(TransportProtocol::Ipv4(IpNextHeaderProtocols::Icmp));

    let (mut tx, mut rx) = match transport_channel(4096, protocol) {
        Ok((tx, rx)) => (tx, rx),
        Err(e) => panic!("error: {:?}", e),
    };
    let mut iter = icmp_packet_iter(&mut rx);

    loop {
        let mut icmp_header:[u8;ICMP_SIZE] = [0;ICMP_SIZE];
        let icmp_packet = create_icmp_packet(&mut icmp_header);
        let timer = Arc::new(RwLock::new(Instant::now()));
        tx.send_to(icmp_packet, target_ip)?;
        match iter.next() {
            Ok((packet,addr)) => match EchoReplyPacket::new(packet.packet()) {
                Some(echo_reply) => {
                    if packet.get_icmp_type() == IcmpTypes::EchoReply {
                        let start_time = timer.read().unwrap();

                        let rtt = Instant::now().duration_since(*start_time);
                        println!(
                            "ICMP EchoReply received from {:?}: {:?}, Time:{:?}",
                            addr,
                            packet.get_icmp_type(),
                            rtt
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
        // std:🧵:sleep(Duration::from_millis(500));
        thread::sleep(Duration::from_millis(500));
    }
}



fn create_icmp_packet<'a>(icmp_header:&'a mut [u8]) -> MutableEchoRequestPacket<'a> {
    let mut icmp_packet = MutableEchoRequestPacket::new(icmp_header).unwrap();
    // icmp_packet.set_icmp_type(IcmpType(8));
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    icmp_packet.set_icmp_code(IcmpCodes::NoCode);
    icmp_packet.set_identifier(random::<u16>());
    icmp_packet.set_sequence_number(1);
    let checksum = util::checksum(icmp_packet.packet(), 1);
    icmp_packet.set_checksum(checksum);

    icmp_packet

}