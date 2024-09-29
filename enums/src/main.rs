#[derive(Debug)]
enum IpAddr {
    V4(IpV4Addr),
    V6(IpV6Addr),
}

#[derive(Debug)]
struct IpV4Addr {
    b1: u8,
    b2: u8,
    b3: u8,
    b4: u8,
}

#[derive(Debug)]
struct IpV6Addr {
    address: String,
}

impl IpAddr {
    fn route(&self) {
        println!("Routing: {:#?}", &self)
    }
}

impl IpV4Addr {
    fn new(b1:u8, b2:u8, b3:u8, b4:u8) -> Self {
        IpV4Addr {
            b1,
            b2,
            b3,
            b4
        }
    }
}

impl IpV6Addr {
    fn new(address: String) -> Self {
        IpV6Addr {
            address
        }
    }
}

fn main() {
    let ipv4_type = IpV4Addr::new(127, 0, 0, 0);
    let ipv6_type = IpV6Addr::new(String::from("::0"));

    let ipv4 = IpAddr::V4(ipv4_type);
    let ipv6 = IpAddr::V6(ipv6_type);

    ipv4.route();
    ipv6.route();

}
