pub fn print_current_number(num: u64) {
    println!("Current number in sequence: {}", num)
}

pub fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}