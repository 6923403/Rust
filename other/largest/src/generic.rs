/*
 * Error: trait 
 */
fn largest_t<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main()
{
    te4();
}

fn te4()
{
    let number_list = vec![34, 10, 39, 78, 49];
    let result = largest_t(&number_list);

    let number_list2 = vec![10, 39, 87, 23, 58, 67];
    let result2 = largest_t(&number_list2);

    let number_list3 = vec!['q', 'f', 'l', 's', 'p', 'u'];
    let result3 = largest_t(&number_list3);

    println!("R1 = {}, R2 = {}, R3 = {}", result result2, result3);
}

