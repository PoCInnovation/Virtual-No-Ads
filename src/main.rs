use std::fs::File;
use std::io;
use std::io::BufRead;
use std::process;
use std::net::IpAddr;

use pnet::datalink::{self, Channel};
use pnet::packet::Packet;
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::ip::IpNextHeaderProtocols;

use dns_lookup::lookup_addr;

fn main()
{
    let mut vec_ads: Vec<String> = Vec::new();

    match parse_adsfile("assets/adsdomain.txt", &mut vec_ads) {
        Ok(_) => {},
        Err(_) => process::exit(84),
    };
    catch_packets(vec_ads);
}

fn parse_adsfile(filename: &str, vec_ads: &mut Vec<String>) -> Result<(), ()>
{
    let file = File::open(filename);

    let file = match file {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error while opening ads_domains file: {error}");
            return Err(());
        }
    };

    let lines = io::BufReader::new(file).lines();

    for line in lines {
        match line {
            Ok(content) => vec_ads.push(content),
            Err(error) => {
                eprintln!("Error while reading content of ads_domain file: {}", error);
                return Err(());
            }
        }
    }
    Ok(())
}

fn catch_packets(vec_ads: Vec<String>)
{
    let interfaces = datalink::interfaces();

    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == "wlo1")
        .expect("Interface not found");

    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error opening channel: {}", e),
    };

    loop {
        let packet = rx.next().unwrap();
        let ethernet = EthernetPacket::new(packet).unwrap();

        if ethernet.get_ethertype() == EtherTypes::Ipv4 {
            let ipv4 = Ipv4Packet::new(ethernet.payload()).unwrap();

            if ipv4.get_next_level_protocol() == IpNextHeaderProtocols::Tcp {
                let tcp = TcpPacket::new(ipv4.payload()).unwrap();
                let src_ipv4 = ipv4.get_source();
                let src_ip = IpAddr::V4(src_ipv4);
                let dst_ip = ipv4.get_destination();
                let src_port = tcp.get_source();
                let dst_port = tcp.get_destination();
                let domain_name = lookup_addr(&src_ip).unwrap_or_else(|_| String::from("Unknown"));

                if vec_ads.iter().any(|element| domain_name.contains(element)) {
                    continue;
                }
                println!("Source IP: {}", src_ipv4);
                println!("Destination IP: {}", dst_ip);
                println!("Source Port: {}", src_port);
                println!("Destination Port: {}", dst_port);
                println!("Domain Name: {}", domain_name);
            }
        }
    }
}
