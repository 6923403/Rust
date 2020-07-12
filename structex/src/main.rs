fn main()
{
    te1();
}

struct Rectangle
{
    width: i32,
    height: i32,
}

fn te1()
{
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45
    };

    println!("rect1 > rect2 ? = {}", rect1.can_hold(&rect2));
    println!("rect1 > rect3 ? = {}", rect1.can_hold(&rect3));
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

