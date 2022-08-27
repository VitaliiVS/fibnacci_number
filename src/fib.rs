use crate::utils::print_current_number;

pub fn fib(number: u64) {
    let mut prev: u64 = 0;
    let mut current: u64 = 1;
    let mut sum:u64 = 0;
    let mut i = 3;

    let mut message: String = String::from("Sequence: ");
    
    if number == current {
        print_current_number(prev)
    }

    if number <= 3 {
        print_current_number(current)
    }

    while i <= number {
        let end: &str = if i == number { "." } else { ", " };

        if i <= 4 {
            message = message + &prev.to_string() + end;

            print_current_number(prev)
        }

        sum = prev + current;
        prev = current;
        current = sum;
        i += 1;

        message = message + &current.to_string() + end;

        print_current_number(current)
    }

    println!("{message}");
    println!("Your number is: {sum}");
}
