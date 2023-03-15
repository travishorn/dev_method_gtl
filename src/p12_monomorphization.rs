// https://youtu.be/JLfEiJhpTbE?t=691

// Monomorphization is the process of turning generic code into specific code by filling in the
// concrete types used at compile time

pub fn main() {
    // If you write this in your code.
    let _integer: Option<i32> = Some(5);
    let _float: Option<f64> = Some(5.0);

    // It gets monomorphized using enums like below.
    enum OptionI32 {
        Some(i32),
        None,
    }

    enum OptionF64 {
        Some(f64),
        None,
    }

    // This is the code that is actually used by the compiler
    let _integer = OptionI32::Some(5);
    let _float = OptionF64::Some(5.0);

    let _integer = OptionI32::None;
    let _float = OptionF64::None;
}
