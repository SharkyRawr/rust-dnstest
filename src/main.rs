use log::{error, warn, info, debug};
use std::net::UdpSocket;
use std::time::Duration;

mod dns;

const test_servers: &'static [&'static str] = &[
    "192.168.6.1:53",
    "8.8.8.8:53",
    "1.1.1.1:53"
];

fn main() {
    println!("Work-in-Progress DNS bench by Sophie 'Sharky🦈' Schumann\n");

    let udp = UdpSocket::bind("0.0.0.0:12345").expect("Could not bind UDP client socket?");
    let my_timeout = Duration::new(5, 0);
    udp.set_read_timeout(Some(my_timeout)).expect("Unable to set read timeout on socket.");
    udp.set_write_timeout(Some(my_timeout)).expect("Unable to set write timeout on socket.");

    let q = dns::DnsQuestion {
        tx: 0xBABE,
        hostname: "shark.pm".to_ascii_lowercase()
    };

    for server in test_servers {
        match q.benchmark(&udp, *server) {
            Some(r) => {
                println!("{} -> {:?}", *server, r)
            },
            None => {
                error!("No result from benchmark. :(");
            }
        }
    }
   
}
