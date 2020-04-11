fn main() {

    /*
    println!("123");

    another_function();
    */

    test2();

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
