fn main()
{
    te1();
    te2();
}


fn te2()
{
    let number_list = vec![11, 20, 30, 99, 32];
    let result2 = largest(&number_list);
    println!("result2 = {}", result2);

}

fn te1() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest1 = number_list[0];

    for number in number_list {
        if number > largest1 {
            largest1 = number;
        }
    }

    let number_list = vec![88, 50, 10, 72, 61];
    let result = largest(&number_list);
    println!("The largest number is {}", largest1);
    println!("Result = {}", result);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
