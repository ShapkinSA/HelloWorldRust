use std::ffi::c_int;

fn main() {

    let mut x = 10;
    let r : &mut i32 = &mut x;


    println!("{r}");
    println!("{x}");

    *r+=1;

    println!("{r}");
    println!("{x}");

}
