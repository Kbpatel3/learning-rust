fn main() {
    println!("Hello, world!");

    // Primitive Types (Scalar Types : Finite set of values)
    // Integer (uint, int), Floating Point (f32, f64), Boolean (bool), Character (char)

    // Integers: i8, i16, i32, i64, i128, and isize (pointer size) (Signed) (-2^(n-1) to 2^(n-1) - 1)
    // Integers: u8, u16, u32, u64, u128, and usize (pointer size) (Unsigned) (0 to 2^n - 1)
    let x: i32 = 2; // 32-bit signed integer (Default)
    let y: u32 = 2; // 32-bit unsigned integer
    //let z: u32 = -2; // Error: Negative integer cannot be assigned to unsigned integer

    println!("\nNow printing x and y:");
    println!("x: {}", x);
    println!("y: {}", y);

    // Floating Point: f32, f64 (Default)
    let a: f32 = 2.0; // 32-bit floating point
    let b: f64 = 2.93; // 64-bit floating point (Default)

    println!("\nNow printing a and b:");
    println!("a: {}", a);
    println!("b: {}", b);

    // Boolean: bool (true, false)
    let true_or_false: bool = true; // 1 is true
    let false_or_true = false;  // 0 is false

    println!("\nNow printing true_or_false and false_or_true:");
    println!("true_or_false: {}", true_or_false);
    println!("false_or_true: {}", false_or_true);

    // Character: char (4 bytes)
    let c: char = 'a';  // Has to be 'single quotes'

    println!("\nNow printing c:");
    println!("c: {}", c);


    // Primitive Types (Compound Types: Group of other values)
    // Tuple, Array

    // Tuple: Fixed length immutable (Once declared, cannot grow or shrink in size)
    let mut tup = (1, true, 's');   // Implicit type declaration (i32, bool, char)
    let tup2: (bool, bool, i32) = (false, true, 2); // Explicit type declaration

    // Accessing tuple elements
    // println!("{}", tup);    // Error: Cannot print tuple directly

    println!("\nNow printing tup:");

    println!("{}", tup.0);  // Accessing first element
    println!("{}", tup.1);  // Accessing second element
    println!("{}", tup.2);  // Accessing third element

    println!("\nNow printing tup2:");

    println!("{}", tup2.0);  // Accessing first element
    println!("{}", tup2.1);  // Accessing second element
    println!("{}", tup2.2);  // Accessing third element

    // Mutating tuple elements
    tup.0 = 10; // Changing first element of tup
    tup.1 = false; // Changing second element of tup
    tup.2 = 't'; // Changing third element of tup
    // tup.3 = 2.0; // Error: Cannot add new element to tuple

    println!("\nNow printing tup after mutation:");
    println!("{}", tup.0);  // Accessing first element
    println!("{}", tup.1);  // Accessing second element
    println!("{}", tup.2);  // Accessing third element

    // Arrays: Fixed length immutable (Once declared, cannot grow or shrink in size)
    let mut arr = [1, 2, 3, 4, 5];  // Implicit type declaration (All elements must be of same type)

    // Accessing array elements
    println!("\nNow printing arr:");
    println!("{}", arr[0]); // Accessing first element
    println!("{}", arr[1]); // Accessing second element
    println!("{}", arr[2]); // Accessing third element
    println!("{}", arr[3]); // Accessing fourth element
    println!("{}", arr[4]); // Accessing fifth element

    // Mutating array elements
    arr[0] = 10; // Possible Error: Cannot mutate array elements if array is declared as immutable

    println!("\nNow printing arr after mutation:");
    println!("{}", arr[0]); // Accessing first element
    println!("{}", arr[1]); // Accessing second element
    println!("{}", arr[2]); // Accessing third element
    println!("{}", arr[3]); // Accessing fourth element
    println!("{}", arr[4]); // Accessing fifth element

    // Declaring array explicitly
    let arr2: [i32; 3] = [1, 2, 3]; // Explicit type declaration [type; size]

    println!("\nNow printing arr2:");
    println!("{}", arr2[0]); // Accessing first element
    println!("{}", arr2[1]); // Accessing second element
    println!("{}", arr2[2]); // Accessing third element
}
