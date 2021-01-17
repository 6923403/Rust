fn main()
{
    let s = String::from("hello world");
    let _sl = first_word(&s);

    println!("_sl = {}", _sl);
}

fn first_word(s: &String) -> &str
{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[0..i];
        }
    }

    &s[..]
}


fn tes1()
{
    let s = String::from("hello");
    let len = s.len();
    let _slice = &s[0..len];
    let _slice = &s[..];

    println!("_slice = {}", _slice);
}
