use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main()
{
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number)
    {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("")
    }

}
