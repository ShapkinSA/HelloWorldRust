use std::ffi::c_int;
use std::os::raw::c_uint;

enum MyEnum {
    First,
    Second,
    Third,
    Fourth, // Once again: trailing comma
}

fn main() {
    let x = MyEnum::Fourth;
    match x {
        MyEnum::First => println!("First"),
        MyEnum::Second => {
            for i in 0..5 { println!("{i}"); }
            println!("Second");
        },
        //всё остальное (если не нашли данного значения из приведённых выше)
        _ => println!("Matched something!"),
    }
}
