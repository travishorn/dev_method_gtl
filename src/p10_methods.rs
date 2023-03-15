// https://youtu.be/JLfEiJhpTbE?t=446

// The `Point` struct has properties `x` and `y`. They are private.
struct Point<T> {
    x: T,
    y: T,
}

// If we want to get `x` for example, we can build a getter

// Since the type of `x` is generic, we have to annotate that generic type here again
impl<T> Point<T> {
    // In this method `x`, return a reference to the value inside the property `x`
    fn x(&self) -> &T {
        &self.x
    }
}

// We can target specific types for various methods
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn main() {
    let integer_point = Point { x: 5, y: 10 };
    println!("Integer point x = {}", integer_point.x());

    let float_point = Point { x: 2.5, y: 5.0 };
    println!(
        "Distance from origin: {}",
        float_point.distance_from_origin()
    );
}
