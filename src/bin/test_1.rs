use std::ffi::c_int;

fn main() {

    let x: c_int = 12;

    println!("Hello, world! {}",x);
    println!("Hello, world! {x}");

}
