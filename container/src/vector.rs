fn te() {
let v: Vec<i32> = Vec::new();

let v2 = vec![1, 2, 3];

let mut v3 = Vec::new();
v3.push(5);
v3.push(6);
v3.push(7);
v3.push(8);


{
    let v4 = vec![1, 2, 3, 4];
    /*
     * deal v4
     */
}/* drop v4 */
}

fn t2()
{
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2)
    {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn te3()
{
    let v1 = vec![100, 32, 57];
    for i in &v1 
    {
        println!("{} ", i);
    }

    let mut v2 = vec![100, 32, 57];
    for i in &mut v2
    {
        *i += 50;
        println!("{} ", i);
    }
}

enum Spread
{
    Int(i32),
    Float(f64),
    Text(String),
}

fn te4()
{
    let row = vec![
    Spread::Int(3),
    Spread::Text(String::from("blue")),
    Spread::Float(10.12),
    ];
}

fn main()
{
    te4();
}

