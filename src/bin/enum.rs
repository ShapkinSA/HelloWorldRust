use std::ffi::c_int;
use std::os::raw::c_uint;

struct Example<T> {
    size : i32,
    data: Vec<T>,
}

impl<T> Example<T> {

    //static function
    pub fn new () -> Self {
        Self {
            size : 2,
            data : Vec::new(),
        }
    }

    //Мутируем объект, на котором вызываем метод (меняем поле)
    pub fn push (&mut self, x:T) {
        self.data.push(x);
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }
}

fn main() {
    let mut vec = Vec::new();
    vec.push(1.0);
    vec.push(2.0);
    let mut ex = Example{ data: vec, size: 666 };
    let y = Example::<i32>::new();
    println!("{}",y.get_size());
    println!("{}",ex.get_size());
    ex.push(700.0);
    println!("{}",ex.get_size());
    // println!("{y}");
    // println!("{x}");
}
