use std::io;

fn main() {
    let mut guess = String::new();

    println!("****Guess The Number****");
    println!("Please input your guess.");

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {guess}");
}
