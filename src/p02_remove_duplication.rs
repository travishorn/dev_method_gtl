// https://youtu.be/JLfEiJhpTbE?t=112

// This is actually an intermediate step that he doesn't exactly cover. In the video, he skips
// straight to using references and array-like datatypes. But I wanted to show what the function
// looks like if you take the code exactly as it was in the last step, but moved into a function.

// Instead of coding two identical `for` loops, we can remove duplication by writing it as a
// function, then calling it each time we want to use it.
fn largest(number_list: Vec<i32>) -> i32 {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

pub fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(number_list);

    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(number_list);

    println!("The largest number is {}", result);
}
