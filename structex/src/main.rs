fn main()
{
    //te1();
    //te2(); 
    te4(); //struct
}

#[derive(Debug)]
struct Rectangle
{
    width: u32,
    height: u32,
}

fn te4()
{
    let rect2 = Rectangle 
    {
        width: 30,
        height: 50
    };

    println!("rect1 is {:?}", rect2);
}

fn te3()
{
    let rect1 = Rectangle
    {
        width: 30,
        height: 50
    };

    println!("The area = {}", area3(&rect1));
}

fn area3(rectangle: &Rectangle) -> u32
{
    rectangle.width * rectangle.height
}

fn te2()
{
    let rect1 = (30, 50);

    println!("The area = {}", area2(rect1));
}

fn area2(wh: (u32, u32)) -> u32
{
    wh.0 * wh.1
}

fn te1()
{
    let width1 = 30;
    let height1 = 50;

    println!("The area = {}", area(width1, height1));

}

fn area(width: u32, height: u32) -> u32
{
    width * height
}

