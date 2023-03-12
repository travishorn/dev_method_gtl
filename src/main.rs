// `x` and `y` values can be any type, but they must be the same type.
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };
    
    let float_point = Point { x: 1.0, y: 4.0 };

    // This will not work because `x` and `y` are defined to be the same type in the struct
    //let mismatched_types = Point { x: 5, y: 4.0 };
}
