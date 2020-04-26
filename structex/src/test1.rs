#[derive(Debug)]
struct Rectangle
{
    width: u32,
    height: u32,
}

impl Rectangle 
{
    fn area(&self) -> u32
    {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle
    {
        Rectangle 
        {
            width: size,
            height: size
        }
    }

}


fn main()
{
    te3();
}

fn te3()
{
    let r3 = Rectangle
    {
        width: 10,
        height: 20
    };

    println!("r3 = {}", r3.width);
    let q = Rectangle::square(20);
    println!("q = {}, {}", q.height, q.width);
}

fn te2()
{
    let r1 = Rectangle
    {
        width: 30,
        height: 50
    };

    let r2 = Rectangle
    {
        width: 10,
        height: 40
    };

    let r3 = Rectangle
    {
        width: 60,
        height: 45
    };

    println!("Can r1 hold r2?  {}", r1.can_hold(&r2));
    println!("Can r1 hold r3?  {}", r1.can_hold(&r3));
}

fn te1()
{
    let rect1 = Rectangle
    {
        width: 30,
        height: 50
    };

    println!("The area  = {}", rect1.area());
}

