// https://youtu.be/JLfEiJhpTbE?t=161

// These two functions are very similar. The only difference is the types they operate on.

// Like before, we can find the largest i32
fn largest_i32(number_list: &[i32]) -> i32 {
    let mut largest = number_list[0];

    for &number in number_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

// We can also now find the largest char with this new function
fn largest_char(char_list: &[char]) -> char {
    let mut largest = char_list[0];

    for &char in char_list {
        if char > largest {
            largest = char;
        }
    }

    largest
}

pub fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);

    println!("The largest number is {}", result);

    let char_list = vec!['a', 'b', 'c', 'd', 'e'];

    let result = largest_char(&char_list);

    println!("The largest char is {}", result);
}
