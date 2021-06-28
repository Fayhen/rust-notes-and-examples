fn main() {
    // Tuples are one of two compound types (the other being arrays).
    // Tuples have fixed size, being unable to grow or shrink.
    // Tuple data is saved on the heap instead of the stack.

    // Tuple declaration
    let tuple_1 = (10, 2, 3.14);
    println!("\nTuples are declared as comma-separated values inside parentheses. For example:");
    println!("{:?}", tuple_1);

    // Tuple typing
    let tuple_2: (String, f64, u8) = ("first".to_string(), 0.5, 1);
    println!("\nTuples may hold different types. These can be inferred dynamically of expressed with type notation.");
    println!("For example, the following tuple has type (String, f64, u8):");
    println!("{:?}", tuple_2);

    // Type mismatch
    // let tuple_3: (u8, String, u8) = (1, 2, 3); // Does not compile
    // let tuple_3: (u8, String, u8) = (1, "2".to_string(), 3); // Compiles

    // Index dot notation
    println!("\nTuple indexes may be accessed with dot notation. For example, using the tuple above:");
    println!("tuple.0 is: {}", tuple_2.0);
    println!("tuple.1 is: {}", tuple_2.1);
    println!("tuple.2 is: {}", tuple_2.2);

    // Destructuring
    let (a, b, c) = tuple_2;
    println!("\nRust also has support for destructuring syntax for tuples.");
    println!("For example, say we destructure the same tuple above with `let (a, b, c) = tuple;`:");
    println!("a is: {}", a);
    println!("b is: {}", b);
    println!("c is: {}", c);
}
