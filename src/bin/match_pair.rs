use std::ffi::c_int;
use std::os::raw::c_uint;

fn main() {

    let number = 3;
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

}
