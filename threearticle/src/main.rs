fn test1(){

    let mut x = 5;
    println!("THe value of x is {}", x);
    
    x = 6;
    println!("The value of x is {}", x);

    const MAX_POINTS: u32 = 100_000;

    let k = 10;
    
    let k = k + 2;

    let k = k + 3;

    println!("The value of k is {}", k);

    let spaces = "  ";
    println!("spaces =  {}", spaces);

    let spaces = spaces.len();
    println!("spaces =  {}", spaces);

}

fn test2()
{
    let _guess: u32 = "42".parse().expect("Not a number");

    let x = 2.2;
    let y: f32 = 3.3;

    println!("x = {}, y = {}", x, y);


}

fn test3()
{
    let sum = 5 + 10;
    let difference = 95.5 - 4.2;

    let product = 4 * 20;
    let quotient = 56.7 / 20.1;

    let remainder = 43 % 5;

    println!("sum = {}, difference = {}, product = {}, quotient = {}, remainder = {}", sum, difference, product, quotient, remainder);

}

fn test4()
{
    let t = true;
    let f: bool = false;

}

fn test5()
{
    let c = 'z';
    let z = 'Z';
    let heart_a = '☯';

    println!("c = {}, z = {}, heart_a = {}", c, z, heart_a);

    loop{
    println!("{}", heart_a);
    }
}

fn test6()
{
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("x = {}, y = {}, z = {}", x, y, z);
}

fn test7()
{
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("five_hundred = {}, six_point_four = {}, one = {}", five_hundred, six_point_four, one);
}

fn test8()
{
    let a = [1, 2, 3, 4, 5];

    let b: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];

    let first = a[0];
    let second = a[1];

    let ten = a[9];

    println!("first = {}, second = {}", first, second);

    println!("ten = {}", ten);

}

fn main()
{
    test8();
}
