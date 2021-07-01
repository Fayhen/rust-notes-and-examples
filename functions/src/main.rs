use std::io;

// A word on statements and expressions:
// Statements are action-performing instructions that do not return a value.
// Expressions are similar, but evaluate to a resulting value.
// Expressions do not end in semicolons.
// Ending expressions with semicolons will turn it into a statement.

fn main() {
    default_printer();
    printer("Pleased to meet you.");

    // Expression creating new scope
    // Try finishing line 15 with a semicolon, turning it into a statement.
    // An error is caused because 'name' is no longer bound to a returned value.
    let my_name = {
        let value = "Rusty";
        value
    };
    println!("I am {}.", my_name);
    
    let username = get_username();
    println!("That's a good name, {}.", username);

}

// Functions are snake_cased
// Functions may be defined after their call
fn default_printer() {
    println!("\nHello there.");
}

// Parameters must have a type declared
fn printer(msg: &str) {
    println!("{}", msg);
}

// Functions finishing in an expression will return the evaluated value
// The function below gets the user's name and return it via expression
// The return value can be typed with an arrow '->'
fn get_username() -> String {
    let input: String = loop {
        let mut input = String::new();
        println!("\nWhat is your name?");

        io::stdin()
            .read_line(&mut input)
            .expect("Sorry, I couldn't understand. Could you tell again?");

        let input = input.trim().to_string();
        break input
    };
    input
}
