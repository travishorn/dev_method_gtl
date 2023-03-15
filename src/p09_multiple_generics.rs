// https://youtu.be/JLfEiJhpTbE?t=379

// `x` and `y` values can be any type, and they can be different types. (T and U are conventions)
struct Point<T, U> {
    x: T,
    y: U,
}

pub fn main() {
    // let integer_point = Point { x: 5, y: 10 };

    // let float_point = Point { x: 1.0, y: 4.0 };

    // This will work now because `x` and `y` are defined to be different types in the struct
    let mismatched_types = Point { x: 5, y: 4.0 };
}
