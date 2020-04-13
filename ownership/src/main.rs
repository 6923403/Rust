fn main()
{
    test1()
}

fn test1()
{
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1 , s2);

}

fn test()
{

    let mut s = String::from("hello");

    s.push_str(",wordld!");

    println!(" {}", s);
}
