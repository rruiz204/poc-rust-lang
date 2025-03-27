enum IP {
    V4(String),
    V6(String),
}

fn display_address(address: IP) {
    match address {
        IP::V4(address) => println!("IPv4: {}", address),
        IP::V6(address) => println!("IPv6: {}", address),
    }
}

pub fn facade() {
    let address_v4: IP = IP::V4(String::from("192.168.10.1"));
    let address_v6: IP = IP::V6(String::from("2001:db8:3333:4444:5555:6666:7777:8888"));

    display_address(address_v4);
    display_address(address_v6);
}