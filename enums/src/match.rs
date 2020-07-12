enum Coin {
    Penny,
    Nickelm,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_ents2(coin: Coin) -> u8{
    match coin {
        Coin:;Penny => {
            println!("Lucky penny");
            1
        },
        Coin:: Nickel => 5,
        Coin:: Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    //_ match all value
    _ => (),
}
fn main() {
    te1();
}
