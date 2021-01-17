use std::fs::File;

fn main()
{
    te2();
}

fn te2()
{
    let f = File::open("hello.txt").expect("Failed to open htllo.txt");
    let f2 = File::open("2hello.txt").expect("f2: Failed to open htllo.txt");
}

fn te1()
{
    let f = File::open("hello.txt").unwrap();
}
