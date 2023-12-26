fn main() {
    // Comparison and Logical operators = <, >, <=, >=, !=, ==, &&, ||, and !
    // Logical operators precedence = !, &&, and ||

    let condition = 2 < 3;
    println!("condition is {}", condition);

    let condition = 2 <= 2;
    println!("condition is {}", condition);

    //let condition = 2 <= 2.2;   // TODO: Error: mismatched types
    let condition = (2 as f32) <= 2.2;   // TODO: Error: mismatched types
    println!("condition is {}", condition);

    // Truth table for && (AND)
    print!("\nAND Truth table\n");
    print!("true && true is {}\n", true && true);
    print!("true && false is {}\n", true && false);
    print!("false && true is {}\n", false && true);
    print!("false && false is {}\n", false && false);

    // Truth table for || (OR)
    print!("\nOR Truth table\n");
    print!("true || true is {}\n", true || true);
    print!("true || false is {}\n", true || false);
    print!("false || true is {}\n", false || true);
    print!("false || false is {}\n", false || false);

    // Truth table for ! (NOT)
    print!("\nNOT Truth table\n");
    print!("!true is {}\n", !true);
    print!("!false is {}\n", !false);

    // Blank line
    print!("\n");

    // If-else-if-else statement
    let number = 3;

    if number > 5 {
        println!("condition was true");
    }
    else if number == 3 {
        println!("number was three");
    }
    else {
        println!("condition was false");
    }

}
