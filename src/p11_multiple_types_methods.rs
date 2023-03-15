// https://youtu.be/JLfEiJhpTbE?t=547

// Different types for each property
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// Implement a method on the `Point` struct
impl<X1, Y1> Point<X1, Y1> {
    // Mix up the Point with data from another point. Take `x` from the original point, but change
    // the `y` to be equal to the given point. Notice the return type.
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn main() {
    // Create 2 points. The `x` and `y` for both of these are different types
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    // Create a 3rd point using `mixup()`
    let p3: Point<i32, char> = p1.mixup(p2);

    // p3.x = 5, p3.y = c
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
