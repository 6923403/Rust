use std::fs::File;
fn main()
{
    te4();
}

fn te4()
{
    let f = File::open("hello.txt");

    let f = match f
    {
        Ok(file) => file,
        Err(error) => 
        {
            panic!("Problem opening the file: {:?}", error)
        },
    };
}

fn te2()
{
    enum Result<T, E>
    {
        Ok(T),
        Err(E),
    }
}

/*
 * fn te3() 
{
    let f = File::open("hello.txt");

    let v: u32 = File::open("hello.txt");
}
*/

fn te() {
    let v = vec![1, 2, 3];

    v[99];
}
