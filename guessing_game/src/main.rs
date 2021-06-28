use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("\nHello! You have opened a number guessing game!");
    println!("\nI have thought of a number between 1 and 100. Try to guess which one it is!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is: {}.", secret_number);

    loop {
        println!("\nPlease input your guess.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("\nYou guessed: {}.", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
