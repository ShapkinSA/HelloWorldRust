use std::ffi::c_int;

fn main() {

    let y = 42;

    let x = if y < 42 {
        345
    }else{
        y + 534
    };
    println!("{y}");
    println!("{x}");
}
