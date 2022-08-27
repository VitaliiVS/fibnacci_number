mod utils;
mod fib;

use std::io;
use crate::fib::fib;

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
                println!("{error}, please enter a number:");
                continue;
            }
        };
    
        fib(num);
        break;
    }
}
