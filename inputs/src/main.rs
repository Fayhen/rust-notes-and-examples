use std::io;

fn parse_input() -> f64 {
    let input: f64 = loop {
        let mut input = String::new();
        println!("\nEnter a number:");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read. Please try again.");

        let _float: f64 = match input.trim().parse::<f64>() {
            Ok(float) => break float,
            Err(_) => {
                println!("Bad input. Please try again.");
                continue
            }
        };
    };

    input
}

fn main() {
    println!("This program will attempt to parse three inputs into floating point numbers (if not already).");

    let a: f64 = loop {
        let mut input_1 = String::new();
        println!("\nEnter a number:");

        io::stdin()
            .read_line(&mut input_1)
            .expect("Failed to read user input. Try again.");
        
        let _a: f64 = match input_1.trim().parse::<f64>() {
            Ok(float) => break float,
            Err(_) => {
                println!("Bad input. Please try again.");
                continue;
            },
        };

    };

    let b: f64 = loop {
        let mut input_2 = String::new();
        println!("\nEnter a number.");

        io::stdin()
            .read_line(&mut input_2)
            .expect("Failed to read user input. Try again.");
        
        let _b: f64 = match input_2.trim().parse::<f64>() {
            Ok(float) => break float,
            Err(_) => {
                println!("Bad input. Please try again.");
                continue;
            },
        };
        
    };

    let c: f64 = parse_input();

    println!("\nThe inputted values were {}, {} and {} respectively.", a, b, c);
}
