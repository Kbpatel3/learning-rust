fn main() {
    // Implicit type (Compiler infers type)
    let x = 4;

    // Explicit type (We tell the compiler the type)
    let mut y: i32 = 5;

    // Print formatted text. Use {} as a placeholder for the variable
    println!("x is : {}", x);
    println!("y is : {}", y);

    // x = 5;  // This will fail because x is immutable
    y = 4;  // This is fine because y is mutable

    // Only y will change because x is immutable
    println!("\nx is : {}", x);
    println!("y is : {}", y);

    let x =  x + 1;  // This is fine because x is overwritten
    println!("\nx is : {}", x);
    println!("y is : {}", y);

    {
        // Shadowing (Different scope)
        let x = x - 2;  // Used x from the outer scope to create a new x for this scope
        println!("\nx is : {}", x);

        let y = "Hello"; // This is fine because it's a different variable
        println!("y is : {}", y);
    }

    // x is still 5 because the x inside the block is a different variable
    println!("\nx is : {}", x);
    // y is going to be 4 because the y inside the block is a different variable
    println!("y is : {}", y);

    // Constants (Similar to immutable variables, but different conventions)
    const MIN_LEGAL_AGE: u32 = 18;
    // MIN_LEGAL_AGE = 19;  // This will fail because constants can't be changed
    // const MIN_LEGAL_AGE: u32 = 19;  // This is fail because you cannot redefine constants
    println!("\nMinimum legal age is : {}", MIN_LEGAL_AGE);
}
