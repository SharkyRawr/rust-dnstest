use log::{error, warn, info, debug};
use std::net::UdpSocket;
use std::time::Duration;

mod dns;

const TEST_SERVERS: &'static [&'static str] = &[
    "192.168.6.1:53",
    "8.8.8.8:53",
    "1.1.1.1:53",
    "9.9.9.9:53",
    "208.67.222.222:53"
];

const TEST_REPITITIONS: u16 = 3;

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

    for server in TEST_SERVERS {
        let mut nresults = 0;
        let mut cumresults = 0;

        for _ in 0..TEST_REPITITIONS {
            match q.benchmark(&udp, *server) {
                Some(r) => {
                    nresults+=1;
                    cumresults += r.as_millis();
                    info!("{} -> {:?}", *server, r)
                },
                None => {
                    error!("No result from benchmark. :(");
                }
            }
            std::thread::sleep(Duration::new(0, 10000000));
        }

        println!("{} -> avg {}ms ({} runs)", *server, cumresults / nresults, TEST_REPITITIONS);

        
    }
   
}
