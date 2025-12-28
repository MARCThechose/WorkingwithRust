use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("guess the number!");
    println!("Please what is your number?");

    let secret_number = rand::rng().random_range(1..=100);
    println!("The secret number is: {secret_number}");
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    let guess: u32 = guess.trim()
        .parse()
        .expect("Please Type a number!");
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
        
    }
}
