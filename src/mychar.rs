// is_alphabetic()
// is_digit()
// to_uppercase()
// to_lowercase()
// is_whitespace()
// is_control()
// to_string()
fn main() {
    let letter: char = 'A';
    let emoji: char = '😊';
    let digit: char = '1';
    println!("{} {} {}", letter, emoji, digit);

    let text = "Hello, the9 World!";
    for ch in text.chars() {
        println!("{}", ch);
        if ch.is_alphabetic() {
            println!("is_alphabetic: {}", ch);
        } else if ch.is_digit(10) {
            println!("is_digit: {}", ch);
            println!("不要用 as isize, 会转成unicode码: {}", ch as isize);
            println!("to_digit(): {}", ch.to_digit(10).unwrap());
        }
        if ch>='a' && ch<='z'  {  // ('a'..='z').contains(&ch)
            println!("is lowercase: {}", ch);
            println!("it's uppercase: {}", ch.to_uppercase());
        }
        match ch {
            'a'..='z' => println!("Lowercase letter"),
            'A'..='Z' => println!("uppercase letter"),
            '0'..='9' => println!("Digit"),
            _ => println!("Special character"),
        }
    }

}
