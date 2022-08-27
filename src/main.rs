mod utils;
mod fib;

use std::io;
use crate::fib::fib;
use crate::utils::capitalize_first_letter;

fn main() {
    println!("Enter a number:");

    loop {
        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Error");
        
        let num: u64 = match num.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                let err: String = capitalize_first_letter(&error.to_string());
                println!("{}, please enter a number:", err);
                continue;
            }
        };
    
        fib(num);
        break;
    }
}
