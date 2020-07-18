use std::net::UdpSocket;
use std::time::Instant;

/// gets the bit at position `n`. Bits are numbered from 0 (least significant) to 31 (most significant).
fn get_bit_at(input: u32, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}

struct DnsQuestion {
    tx: u16,
    hostname: String
}

impl DnsQuestion {
    fn tobuf(&self) -> Vec<u8> {
        let mut r: Vec<u8> = Vec::new();

        // build header

        // 16 bits transaction/id
        r.extend_from_slice(&self.tx.to_be_bytes());

        //  0  1234  6  7  8
        // QR OPCODE AA TC RD

        r.push(0b00000001);

        //  9 10-11-12 13-14-15-16
        // RA    Z        RCODE
        r.push(0b00000000);

        // QDCOUNT         an unsigned 16 bit integer specifying the number of
        //                 entries in the question section.
        r.extend_from_slice(&(1 as u16).to_be_bytes());

        // ANCOUNT         an unsigned 16 bit integer specifying the number of
        //                 resource records in the answer section.
        r.extend_from_slice(&(0 as u16).to_be_bytes());

        // NSCOUNT         an unsigned 16 bit integer specifying the number of name
        //                 server resource records in the authority records
        //                 section.
        r.extend_from_slice(&(0 as u16).to_be_bytes());

        // ARCOUNT         an unsigned 16 bit integer specifying the number of
        //                 resource records in the additional records section.
        r.extend_from_slice(&(0 as u16).to_be_bytes());

        // Add question(s)

        //  QNAME
        //  a domain name represented as a sequence of labels, where
        //  each label consists of a length octet followed by that
        //  number of octets.  The domain name terminates with the
        //  zero length octet for the null label of the root.  Note
        //  that this field may be an odd number of octets; no
        //  padding is used.

        let parts = self.hostname.split('.');
        for part in parts {
            // @todo split parts larger than 255
            let len = part.len() as u8;
            r.push(len);
            r.extend_from_slice(part.to_ascii_lowercase().as_bytes());
        }
        r.push(0b0 as u8);

        //  QTYPE           a two octet code which specifies the type of the query.
        //                  The values for this field include all codes valid for a
        //                  TYPE field, together with some more general codes which
        //                  can match more than one type of RR.
        r.extend_from_slice(&(1 as u16).to_be_bytes());

        //QCLASS          a two octet code that specifies the class of the query.
        //                For example, the QCLASS field is IN for the Internet.
        r.extend_from_slice(&(1 as u16).to_be_bytes());

        r
    }

    pub fn send_to(&self, udp: &UdpSocket, destination: &str) {
        let buf = self.tobuf();
        let mut resp_buf: [u8; 1500] = [0; 1500];

        println!("Sending bytes: {:x?}", buf);
        udp.send_to(&buf, destination).unwrap();
        let start = Instant::now();

        let (num_recv, recv_from) = udp.recv_from(&mut resp_buf).expect("No response?");
        let time_took = start.elapsed();

        println!("Received response: {} bytes from {}, took {:?}", num_recv, recv_from.to_string(), time_took);
    }
}

fn main() {
    println!("Work-in-Progress DNS bench by Sophie 'Sharky🦈' Schumann\n");

    let udp = UdpSocket::bind("0.0.0.0:12345").expect("Could not bind UDP client socket?");

    let q = DnsQuestion {
        tx: 0xBABE,
        hostname: "shark.pm".to_ascii_lowercase()
    };
    q.send_to(&udp, "192.168.6.140:53");
}
