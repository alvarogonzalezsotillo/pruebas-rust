https://www.quantamagazine.org/the-map-of-mathematics-20200213/
use std::env;

pub mod crossteaser;





fn main() {
    let args : Vec<String> = env::args().collect();
    println!("Los argumentos son: {:?}", args);
}




