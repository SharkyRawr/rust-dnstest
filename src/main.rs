use std::net::UdpSocket;

struct DnsQuestion {
    tx: u16,
    hostname: String
}

impl DnsQuestion {
    fn tobuf(&self) -> Vec<u8> {
        let mut r: Vec<u8> = Vec::new();
        let mut bitbuf: u8 = 0;

        // build header

        // 16 bits transaction/id
        r.extend_from_slice(&self.tx.to_be_bytes());

        // 8 bits |QR|   Opcode  |AA|TC|RD
        bitbuf = 0b00000101;
        r.push(bitbuf);

        // 8 bits RA|   Z    |   RCODE   
        bitbuf = 0b00000000;
        r.push(bitbuf);

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

        // @todo labels longer than 255
        let len = self.hostname.len() as u8;
        r.push(len);
        r.extend_from_slice(self.hostname.as_bytes());
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

        print!("Sending bytes: {:x?}", buf);
        udp.send_to(&buf, destination).unwrap();
    }
}

fn main() {
    println!("Work-in-Progress DNS bench by Sophie 'Sharky 🦈' Schumann");

    let udp = UdpSocket::bind("0.0.0.0:12345").expect("Could not bind UDP client socket?");

    let mut q = DnsQuestion {
        tx: 1337,
        hostname: "shark.pm".to_string()
    };
    q.send_to(&udp, "1.1.1.1:53");
}
