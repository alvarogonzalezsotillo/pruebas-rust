
use std::env;

pub mod crossteaser;





fn main() {
    let args : Vec<String> = env::args().collect();
    println!("Los argumentos son: {:?}", args);
}




