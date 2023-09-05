use std::ffi::c_int;

fn main() {

    let mut x = 10;
    for i in 0..5 {
        if(x==10){
            println!("{i}");
            // let x = 12;
            x = 12;
        }
    }
}
