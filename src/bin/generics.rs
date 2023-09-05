use std::ffi::c_int;
use std::os::raw::c_uint;

struct Example {
    count: i32,
    size : i32,
}


impl Example {

    //static function
    pub fn new () -> Self {
        Self {
            count:2,
            size:8,
        }
    }

    //Мутируем объект, на котором вызываем метод (меняем поле)
    pub fn change (&mut self, x:i32) {
        self.count = x;
    }


    pub fn get_count(&self) -> i32 {
        self.count
    }


}

fn main() {

    let mut ex = Example{ count: 123, size: 666 };

    let y = Example::new();

    println!("{}",y.get_count());
    println!("{}",ex.get_count());

    ex.change(700);

    println!("{}",ex.get_count());


    // println!("{y}");
    // println!("{x}");
}
