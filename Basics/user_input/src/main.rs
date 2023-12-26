use std::io;

fn main() {
    println!("Hello, world!");

    // String object
    let mut input = String::new();

    // Read line from stdin passing a mutable reference to the String object input
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Print the input
    println!("You typed: {}", input);
}
