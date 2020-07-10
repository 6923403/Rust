fn main()
{
    first();
}

fn first_word(ss: &str) -> &str 
{
    let bytes = ss.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &ss[0..i];
        }
    }

    &ss[..]
}

fn first()
{
    let ss = String::from("QWE RTY");

    let re = first_word(&ss[..]);

    println!("re = {}", re);

    /*
     * let a = [1, 2, 3, 4, 5, 6];
    let slice3 = &a[1..3];

    println!("slice = {}", slice3);
    */
}

fn te2()
{
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice2 = &s[..2];

    println!("slice = {}, slice2 = {}", slice, slice2);
}

fn te1()
{
    let s = String::from("Hello World");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello = {}, world = {}", hello, world);

}
