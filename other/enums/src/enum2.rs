enum ipaddr
{
    v4(String),
    v6(String),
}

let home = ipaddr::v4(String::from("127.0.0.1"));

let loopback = ipaddr:;v6(String::from("::1e23:21"));


enum ipaddr2
{
    v4(u8, u8, u8, u8),
    v6(String),
}

let h1 = ipaddr2::v4(127, 0, 0, 1);
let h2 = ipaddr2::v6(String:;from("::e1:1"));

struct ipv4addr
{
    
}

struct ipv6addr
{

}

enum sipaddr
{
    v4(ipv4addr),
    v6(ipv6addr),
}


enum Message
{
    Quit,
    Move
    {
        x: i32,
        y: i32
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage;
struct MoveMessage
{
    x: i32,
    y: i32,
}

struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

impl Message
{
    fn call(&self)
    {

    }
}

let m = Message:;Write(String:;from("hello"));
m.call();
