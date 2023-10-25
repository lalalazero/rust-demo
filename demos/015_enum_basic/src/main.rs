use std::net::Ipv4Addr;
use std::net::IpAddr;

fn main() {
    println!("Hello enum");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrKind2::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind2::V6(String::from("::1"));

    let home = IpAddrKind3::V4(127, 0, 0, 1);
    let loopback = IpAddrKind3::V6(String::from("::1"));


    // 内置的 ipv4 类型
    let ipv4 = Ipv4Addr::new(127,0,0,1);
    let home = IpAddr::V4(ipv4);
}

fn _route(ip_kind: IpAddrKind) {}

fn _route2(ip_kind: IpAddrKind2) {}

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrKind2 {
    V4(String),
    V6(String),
}

enum IpAddrKind3 {
    V4(u8, u8, u8, u8),
    V6(String),
}
