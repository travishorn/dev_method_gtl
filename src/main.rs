// Now, the generic type `T` given to `largest()` must implement the `PartialOrd` trait. This way,
// Rust is guaranteed to know how to handle the comparison operator `>`.

// The return type must be `&T` now. It returns a reference to the largest element in the slice
// rather than the element itself because moving a non-copy type out of a container like a slice is
// not allowed.
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    // This variable must also be changed to a reference `&list[0]` because the function now returns
    // a reference to the largest element, so it needs to keep track of a reference to the largest
    // element instead of keeping track of the largest element's value itself.
    let mut largest = &list[0];

    // Finally, since the function now takes a reference to a slice, each item in the loop is
    // already a reference to an element. We use `item` rather than `&item` because the latter would
    // create a reference to the reference.
    for item in list {
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
