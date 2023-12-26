use std::io;

fn main() {
    let x: u8 = 9;  // 8-bit unsigned integer (0 to 255)
    let y: i8 = 10; // 8-bit signed integer (-128 to 127)

    let j: i8 = 9;  // 8-bit signed integer (-128 to 127)
    let k: i8 = 10; // 8-bit signed integer (-128 to 127)

    let a: i8 = 9;  // 8-bit signed integer (-128 to 127)
    let b: i64 = 10; // 64-bit signed integer (-9223372036854775808 to 9223372036854775807)

    let c: u8 = 255; // 8-bit unsigned integer (0 to 255)
    let mut d: u8 = 1;   // 8-bit unsigned integer (0 to 255)


    // Addition (x and y are not the same type)
    //let z = x + y; // TODO: error[E0308]: mismatched types
    let z = (x as i8) + y; // Explicit Type Casting (will work because x is casted to i8
    println!("{} + {} = {}", x, y, z);

    // Addition (j and k are the same type)
    let z = j + k;
    println!("{} + {} = {}", j, k, z);

    // Addition (a and b are not the same type)
    // let z = a + b;   // TODO: error[E0308]: mismatched types
    let z = (a as i64) + b; // Explicit Type Casting (will work because a is casted to i64. Always cast to the larger data type)
    println!("{} + {} = {}", a, b, z);

    // Addition (will not work because the data type is unsigned and the result is greater than 255) (Overflow)
    // let z = c + d;
    let z = c as u32 + d as u32; // Explicit Type Casting (will work because c and d are casted to u32)
    println!("{} + {} = {}", c, d, z);

    // Subtraction (will not work because the data type is unsigned and the result is less than 0) (Underflow)
    // let z = d - c;
    let z = d as i32 - c as i32; // Explicit Type Casting (will work because d and c are casted to i32 which allows negative numbers)
    println!("{} - {} = {}", d, c, z);

    // Division (will work because the data type is unsigned) (255 / 10 = 25 and not 25.5 because of the data type)
    d = 10;
    let z = c / d;
    println!("{} / {} = {}", c, d, z);

    // Division (will not work because the operands divided result in u8 and not f32)
    // let z: f32 = c /d;
    let z: f32 = c as f32 / d as f32; // Explicit Type Casting (will work because c and d are casted to f32)
    println!("{} / {} = {}", c, d, z);

    // Multiplication (will work because the data type is unsigned and the result is less than or equal to 255)
    d = 1;
    let z = c * d;
    println!("{} * {} = {}", c, d, z);

    // Modulus (will work because the data type is unsigned and the result is less than 255)
    d = 10;
    let z = c % d;
    println!("{} % {} = {}", c, d, z);

    // Creating a string
    let mut input = String::new();

    // Read a line into input
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Convert input to an integer
    let int_input: i64 = input.trim().parse().expect("Please type a number!"); // trim() removes the newline character from the input

    // Print the input
    println!("You typed: {}", int_input);
    println!("{} + 1 = {}", int_input, int_input + 1);

}
