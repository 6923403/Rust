enum ipv
{
    v4,
    v6,
}

struct ipaddr
{
    kind: ipv,
    address: String,
}

let home = ipaddr
{
    kind: ipv::v4,
    address: String::from("127.0.0.1"),
}

let loopback = ipaddr
{
    kind: ipv::v6,
    address: String::from("::eee:1"),
}

fn main() {
    te1();
}

fn te1()
{
    let four = ipv::v4;
    let six = ipv::v6;
}

fn route(ip_type: ipv)
{}
