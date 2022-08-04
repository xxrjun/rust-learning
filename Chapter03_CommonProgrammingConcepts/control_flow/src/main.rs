fn main() {
    /*
        1. if expressions
    */
    let age = 20;

    if age < 18 {
        println!("Your age is {}. You are not adult.", age);
    } else {
        println!("Your age is {}. You are adult.", age);
    }
    println!();

    /*
        2. Using if in a let statement
    */

    let condition: bool = true;

    // `if` and `else` must have compatible types
    let number = if condition { 5 } else { 6 };
    println!("number: {}", number);

    // error[E0308]: `if` and `else` have incompatible types
    // let number = if condition {5} else {"6"}
}
