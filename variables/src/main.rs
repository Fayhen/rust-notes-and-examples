fn main() {
    // Immutable variable
    let x = 3;
    println!("The value of x is {}.", x);
    // x = 4; // Will cause a compiler error

    //Mutable variable
    let mut y = 5;
    println!("The value of y is {}.", y);
    y = 6;
    println!("The value of y is {}.", y);

    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("The value of the MAX_POINTS constant is {}.", MAX_POINTS);

    // Shadowing

    // Tranformations without loosing immutability
    let z = 7;
    let z = z + 1;
    let z = z * 2;
    println!("The value of z is {}.", z);

    // Type change
    let spaces = "     ";
    let spaces = spaces.len();
    println!("The value of spaces is {}.", spaces);

    // Type preservation in mutable variables
    let mut mut_spaces = "    ";
    println!("The value of mut_spaces is {}.", mut_spaces);
    mut_spaces = "     ";
    println!("The value of mut_spaces is {}.", mut_spaces);
    //mut_spaces = mut_spaces.len(); // Will cause a compiler error
}
