fn main() {
    // -----------------------------------------------------------------

    /*
        1. Mutable vs Immutable variable
    */

    // Immutable variable
    let my_immut_var = 5;
    println!("my_immut_var's value is: {}", my_immut_var);

    // error: cannot assign twice to immutable variable
    // my_immut_var = 6;

    // Mutable variable
    let mut my_mut_var = 5;
    println!("my_mut_var's value is: {}", my_mut_var);

    my_mut_var = 6;
    println!("my_mut_var's value is: {}", my_mut_var);
    println!();

    // -----------------------------------------------------------------

    /*
        2. Integer in Rust
    */

    // Unsigned 8bits
    let mut my_u8_integer: u8 = 25;
    println!("my u8 integer is {}", my_u8_integer);

    my_u8_integer = 253; // in u8 valid range
    println!("my_u8_integer is {}", my_u8_integer);

    // Overflow
    // my_integer = 257;

    println!();

    // -----------------------------------------------------------------

    /*
        3. Float in Rust
    */

    let my_float64 = 2.0;

    let my_float32: f32 = 3.0;

    println!("my_float64 is {}", my_float64);
    println!("my_float32 is {}", my_float32);
    println!();

    // -----------------------------------------------------------------

    /*
        4. Numeric Operation
    */

    let sum = 5 + 10;

    let difference = 96 - 4;

    let product = 4 * 25;

    let quotient = 56.5 / 13.2;
    let floored = 2 / 3;

    let remainder = 40 * 3;

    println!(
        "sum: {}, difference: {},\nproduct: {}, quotient: {}, floored: {},\nremainder: {}",
        sum, difference, product, quotient, floored, remainder
    );
    println!();

    // -----------------------------------------------------------------

    /*
        5. Boolean
    */
    let t = true;

    let f: bool = false;

    println!("t: {}, f: {}", t, f);
    println!();

    // -----------------------------------------------------------------

    /*
        6. Character
    */
    let c = 'z';
    let heart_eyed_cat = '😻';

    println!("c: {}", c);
    println!("emoji: {}", heart_eyed_cat);
    println!();

    // -----------------------------------------------------------------

    /*
        7. Tuples
    */

    // 元組的每一格都是一個獨立型別，不同數值不必是相同型別
    let tup: (i32, f32, u16, i64) = (500, 6.4, 1, -2);

    // Desturction
    let (a, b, c, d) = tup;

    println!("c in tuple is: {}", c);

    // Getting element in tuple by index
    let first_element = tup.0;
    let fourth_element = tup.3;

    println!("First element in tuple is: {}", first_element);
    println!("Fourth element in tuple is: {}", fourth_element);

    // No field '5' on type
    // let out_of_bound = tup.5;

    println!();

    // -----------------------------------------------------------------

    /*
        8. Array
    */

    let arr = [1, 2, 3, 4, 5, 6];

    println!("first of array: {}", arr[0]);

    // arr: [type; number of elements]
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // arr = [initail value of each element; number of elements]
    // same as `let arr = [10, 10, 10, 10, 10];`
    let arr = [10; 5];
}
