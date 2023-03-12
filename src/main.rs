// We can reuse the same function for many types using a generic type. Warning: There is more work
// to be done before this function works, however. Rust will complain that it cannot use the
// comparison operator `>` here because it doesn't know how to compare the order of every possible
// type you could give it.
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let char_list = vec!['a', 'b', 'c', 'd', 'e'];

    let result = largest(&char_list);

    println!("The largest char is {}", result);
}
