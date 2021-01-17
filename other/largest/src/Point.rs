struct Point<T> {
    x: T,
    y: T,
}

struct _Point<T, U> {
    x: T,
    y: U,
}

fn te3()
{
    let both_int = _Point{x: 1, y: 10};
    let both_float = _Point{x: 2.1, y: 2.2};
    let intandfloat = _Point{x: 12, y:1.2};
}

fn te2()
{
    /*
     * error type <T> y: float T = int;
     */
    let wont_work = _Point{x: 5, y:4.0};
}

fn te1()
{
    let integer = Point{x: 5, y: 19};
    let float = Point{x: 1.0, y: 2.0};
}

fn main()
{
    te3();
}
