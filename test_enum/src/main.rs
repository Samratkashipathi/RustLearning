/* Method 1 of using enums*/
// #[derive(Debug)]
// enum IpAddressType {
//     Ipv4,
//     Ipv6,
// }

// #[derive(Debug)]
// struct Internet {
//     ipaddress: IpAddressType,
//     address: String,
// }

// fn main() {
//     let four = Internet {
//         ipaddress: IpAddressType::Ipv4,
//         address: String::from("255::255::255::255"),
//     };

//     let six = Internet {
//         ipaddress: IpAddressType::Ipv6,
//         address: String::from("::255"),
//     };

//     println!("Four:{:#?},Six{:#?}", four, six);
// }

/* Method 2 of using enums*/
#[derive(Debug)]
enum IpAddressType {
    Ipv4(String),
    Ipv6(String),
}

fn main() {
    let four = IpAddressType::Ipv4(String::from("127.0.0.1"));
    let six = IpAddressType::Ipv6(String::from("127.0.0.2"));

    println!("Four: {:#?} \n Six: {:#?}", four, six);

    let _test_null : Option<String> = None;
}
