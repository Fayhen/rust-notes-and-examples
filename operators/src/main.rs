use std::io;

fn parse_input() -> f64 {
    let parsed_input: f64 = loop {
        let mut input = String::new();
        println!("\nPlease enter a number:");

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

    parsed_input
}

fn calc_sum(x: f64, y: f64) {
    let result = x + y;
    println!("Sum is: {}", result);
}

fn calc_diff(x: f64, y: f64) {
    let result = x - y;
    println!("Difference is: {}", result);
}

fn calc_prod(x: f64, y: f64) {
    let result = x * y;
    println!("Product is: {}", result);
}

fn calc_div(x: f64, y: f64) {
    let result = x / y;
    println!("Quotient is: {}", result);
}

fn calc_remd(x: f64, y: f64) {
    let result = x % y;
    println!("Remainder is: {}", result);
}

fn calc_less_than(x: f64, y: f64) {
    let result = x < y;
    println!("X less than Y is: {}", result);
}

fn calc_less_equal(x: f64, y: f64) {
    let result = x <= y;
    println!("X less than or equal than Y is: {}", result);
}

fn calc_greater_than(x: f64, y: f64) {
    let result = x > y;
    println!("X greater than Y is: {}", result);
}

fn calc_greater_equal(x: f64, y: f64) {
    let result = x >= y;
    println!("X less than or equal to Y is: {}", result);
}

fn calc_equal_to(x: f64, y: f64) {
    let result = x == y;
    println!("X equal to Y is: {}", result);
}


fn main() {
    println!("\nThis program will go through a few basic arithmetic operations.");
    println!("Please input two numbers to perform operations with.");
    println!("Inputs will be parsed as floating point variables.");

    let x: f64 = parse_input();
    let y: f64 = parse_input();

    println!("\nVariables 'X' and 'Y' we assinged as {} and {} respectively.\n", x, y);

    calc_sum(x, y);
    calc_diff(x, y);
    calc_prod(x, y);
    calc_div(x, y);
    calc_remd(x, y);
    calc_less_than(x, y);
    calc_less_equal(x, y);
    calc_greater_than(x, y);
    calc_greater_equal(x, y);
    calc_equal_to(x, y);
}
