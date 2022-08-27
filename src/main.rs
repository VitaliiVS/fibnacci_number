use std::io;

fn main() {
    let mut num = String::new();

    println!("Enter a number:");

    io::stdin()
        .read_line(&mut num)
        .expect("Error");

    let num: i32 = num.trim()
        .parse()
        .expect("Enter a number");

    fib(num);
}

fn fib(number: i32) {
    let mut prev: i32 = 0;
    let mut current: i32 = 1;
    let mut sum:i32 = 0;
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

    return println!("Your number is: {sum}")
}

fn print_current_number(num: i32) {
    println!("Current number in sequence: {}", num)
}