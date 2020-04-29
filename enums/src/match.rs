enum Coin
{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8
{
    match coin
    {
        Coin::Penny => 
        {
            println!("LQ");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)]
enum UsState
{
    Alabama,
    Alaska,
}

enum Coins
{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cent2(Coins: Coins) -> u8
{
    match coin
    {
        Coins::Penny => 1,
        Coins::Nickel => 5,
        Coins::Dime => 10,
        Coins::Quarter(state) =>
        {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>
{
    match x
    {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main()
{
    te2();
}

fn te2()
{
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn te1()
{
  let q = value_in_cents(Coin::Penny);

  let q2 = value_in_cent2(Coins::Quarter(UsState::Alaska));
}
