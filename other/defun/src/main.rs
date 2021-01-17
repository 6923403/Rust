fn main() {

    /*
    println!("123");

    another_function();
    */

    test8();

}

fn test8()
{
    let a = [10, 20, 30, 40, 50];

    for num in a.iter()
    {
        println!("The value is: {}", num);
    }

    test9();
}

fn test9()
{
    for number in (1..4).rev()
    {
        println!("{}!", number);

    }

    println!("EOF");
}

fn test7()
{
    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5
    {
        println!("The value is: {}", a[index]);

        index = index + 1;
    }
}

fn test6()
{
    let mut number = 3;
    while number != 0
    {
        print!("{}!", number);

        number = number - 1;

    }

    println!("");

    println!("LIFTOFF");
}

fn test5()
{
    let mut counter = 0;

    let result = loop
    {
        counter += 1;

        if counter == 10{
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn test4()
{
    let condition = true;
    let number = if condition{
        5
    }
    else
    {
        6
    };

    println!("The value of number is: {}", number);

}

fn test3()
{
    let number = 3;

    if number % 4 == 0
    {
        println!("true");
    }
    else
    {
        println!("false");
    }
}

/*
fn another_function()
{
    println!("456");
}*/

fn five() -> i32
{
    5
}

fn test1()
{
    /*
    fun1(5);

    fun2(7, 8);
    */
    fun3();

    let x = five();

    println!("x = {}", x);
}

fn fun1(x: i32)
{
    println!("The value of x = {}", x);
}

fn fun2(x: i32, y: i32)
{
    println!("The value of x = {}", x);
    println!("The value of y = {}", y);
}

fn fun3()
{
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("x = {}, y = {}", x, y);
}

fn test2()
{
    let x = plus_one(5);
    println!("The value of x is: {}", x);

}

fn plus_one(x: i32)-> i32
{
    x + 1
}
