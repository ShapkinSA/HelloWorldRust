use std::ffi::c_int;

fn main() {

    let pair: (f32,f32) = (0.1,6.92);

    let x = pair.0;
    let y = pair.1;
 
    println!("{x}");
    println!("{y}");

}
