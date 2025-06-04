use pnet::{datalink, packet::ethernet::{
    EthernetPacket}};
    use pnet::packet::Packet;
    use pnet::packet::FromPacket;

fn main() {
    let interface = pnet::datalink::interfaces();
    // wlp4s0
    let interface = interface[6].clone();

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx,rx )) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type: {}", &interface),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e),
    };

    println!("Start reading packets: ");

    loop {
        match rx.next() {
            Ok(p)=>{
                if let Some(ethernet_packet) = EthernetPacket::new(p) {
                    println!("New packet: ");
                    println!("{:?}",ethernet_packet);

                    let packet = ethernet_packet.packet();
                    let payload = ethernet_packet.payload();
                    let from_packet = ethernet_packet.from_packet();
                    //println!("---");
                    println!("packet: {:?}", packet);
                    // print the full packet as an array of u8
                    println!("payload: {:?}", payload);
                    // print the payload as an array of u8
                    println!("from_packet: {:?}", from_packet);
                    // print the hearder infos: mac address, ethertype, ...
                    // and the payload as an array of u8
                    println!("---");


                }
            },
            Err(e) => {
                panic!("An error occurred while reading:{}",e);
            }
        }
    }
}