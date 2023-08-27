use std::io;

fn main() {
    print!("Hello, world!");
    print!("I'm a Rustacean!"); // no newline is printed!
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    print!("you guessed: {guess}")
    
}
