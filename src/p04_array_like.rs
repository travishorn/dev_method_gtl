// https://youtu.be/JLfEiJhpTbE?t=112

// We can accept any type of array-like data structure by using a slice. This may be preferable
// because it is more flexible than the last version that could only take a vector.
fn largest(number_list: &[i32]) -> i32 {
    let mut largest = number_list[0];

    for &number in number_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

pub fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);

    println!("The largest number is {}", result);
}
