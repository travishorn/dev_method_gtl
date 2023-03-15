// https://youtu.be/JLfEiJhpTbE?t=112

// This is yet another intermediate step he doesn't cover in the video. He skips straight to the
// next step where the function takes any array-like datatype. But I wanted to show what the code
// looks like when it's just working on vectors alone.

// We can be more memory efficient by taking `number_list` as a reference rather than taking
// ownership over it. In the last version, an entire copy of the vector needed to be made. Since
// we're not mutating `number_list` at any time, taking it by reference like you see here is
// preferable.
fn largest(number_list: &Vec<i32>) -> i32 {
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
