use std::net::IpAddr;
use std::collections::HashSet;
use std::fs::read_to_string;

use pnet::datalink::{self, Channel};
use pnet::packet::Packet;
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::ip::IpNextHeaderProtocols;

use dns_lookup::lookup_addr;
use trust_dns_resolver::config;
use trust_dns_resolver::proto::rr::domain;

pub fn parse_adsfile(arguments: &Vec<String>, domains_list: &mut HashSet<String>) -> Result<(), ()>
{
    for i in 2..arguments.len() {
        let file = read_to_string(&arguments[i]).map_err(|error| {
            eprintln!("Error while opening ads_domains file: {}", error);
        })?;

        for content in file.lines() {
            if content.is_empty() {
                continue;
            }
            domains_list.insert(content.to_string());
        }
    }
    Ok(())
}

fn config_hosts(domain_name: &str) -> Result<(), ()> {
    let hosts_path = "/etc/hosts";
    let domains_to_block: HashSet<&str> = vec!["ads.example.com", "adserver.net"].into_iter().collect();
    let block_ip = IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1));

    let current_content = read_to_string(&hosts_path).map_err(|error| {
        eprintln!("Error while opening hosts file: {}", error);
    })?;

    let mut new_content = String::new();
    for line in current_content.lines() {
        if line.trim().is_empty() {
            new_content.push_str("\n");
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let domain = parts[1];
            
            if domain_name == domain {
                let new_line = format!("{} {}\n", block_ip, domain);
                new_content.push_str(&new_line);
            } else {
                new_content.push_str(&line);
                new_content.push_str("\n");
            }
        } else {
            new_content.push_str(&line);
            new_content.push_str("\n");
        }
    }
    Ok(())
}

pub fn catch_packets(interface_name: &str, blacklist: HashSet<String>)
{
    let interfaces = datalink::interfaces();

    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == interface_name)
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

                if src_ipv4.is_loopback() || src_ipv4.is_link_local() {
                    continue;
                }
                if blacklist.contains(&domain_name) {
                    config_hosts(domain_name);
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
