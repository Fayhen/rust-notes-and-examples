fn main() {
    // Arrays are one of two compound types (the other being tupless).
    // Array data is allocated on the stack rather than the heap.
    // Like tuples, arrays in Rust have fixed size and can't shrink or grow.
    // For variable size compound structures, it is possible to use the vector type.

    // Declaration and properties
    let array_1 = [1, 2, 3, 4, 5];
    println!("\nArrays are declared as comma-separated values inside square brackets. For example:");
    println!("{:?}", array_1);

    let array_2 = ["a", "b", "c"];
    println!("\nArrays have fixed size, and all elements must be of the same type, such as: {:?}", array_2);

    let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    println!("\nTherefore, arrays in Rust are a good choice for situations where a program must remember a set of values of the same type, that is unlikely to change. For example:");
    println!("\nDays on the week: {:?}", days);
    println!("\nMonth abbreviations: {:?}", months);

    // Type declaration
    let digits: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("\nType and size may be declared explicitly with the notation: [type; size].");
    println!("This array was declared as \"[u8, 10]\": {:?}", digits);

    // Indexing
    println!("\nSimple indexing notation ([i]) may used to access elements within an array. Take the months array above as an example:");
    println!("months[0] yields: {}", months[0]);
    println!("months[1] yields: {}", months[1]);
    println!("months[2] yields: {}", months[2]);
    println!("months[3] yields: {}", months[3]);
    println!("months[4] yields: {}", months[4]);
    println!("months[5] yields: {}", months[5]);
    println!("months[6] yields: {}", months[6]);
    println!("months[7] yields: {}", months[7]);
    println!("months[8] yields: {}", months[8]);
    println!("months[9] yields: {}", months[9]);
    println!("months[10] yields: {}", months[10]);
    println!("months[11] yields: {}", months[11]);
    
    println!("\nRust will panic at runtime if attempting to access out-of-bounds indexes. This behavior prevents invalid memory access, which may happen in other low-level languages, and is an example of one of Rust's safety principles.");
    // println!("months[11] yields: {}", months[12]); // Either does not compile or panics at runtime
}
