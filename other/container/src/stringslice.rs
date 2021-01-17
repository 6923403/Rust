fn main()
{
    te4();
}

fn te4()
{
    for b in "नमस्ते".bytes() {
    println!("{}", b);
    }
}

fn te3()
{
    let q1 = String::from("Hkkk");
    let h = &q1[0..1];
    println!("{}", h);
}

fn te2()
{
    let mut s = String::from("foo");
    s.push_str("bar");

    println!("{}", s);

    let s1 = String::from("H");
    let s2 = String::from("i!");
    //let s3 = s1 + &s2;

    //println!("s3 = {}", s3);

    let s4 = String::from("qqqqq");
    let st = format!("{}-{}-{}", s1, s2, s4);
    println!("{}", st);
}

fn te1()
{
    let mut s5 = String::new();
    let data = "initial contents";
    let s = data.to_string();
    println!("{}, \n", s);

    let s = "initial contents".to_string();
    let s2 = "initial contents".to_string();

    println!("{}, {}", s, s2);
}

