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

fn main()
{
    te1();
}

fn te1()
{
  let q = value_in_cents(Coin::Penny);

  let q2 = value_in_cent2(Coins::Quarter(UsState::Alaska));
}
