fn main()
{
    test5();
}

fn test5()
{
    let k = dangle();

    println!("{} ", k);
}

fn dangle() -> String
{
    let s = String::from("aa");

    s
}

fn test4()
{
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;

    println!("{}", r3);
}

fn test3()
{
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    }

    let r2 = &mut s;
}

fn test2()
{
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String)
{
    some_string.push_str(",world");
}

fn test1()
{
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

}

fn calculate_length(s: &String) -> usize
{
    s.len()
}
