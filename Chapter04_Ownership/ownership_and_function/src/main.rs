fn main() {
    let s = String::from("Hello"); // s comes in scope

    takes_ownership(s); // s's value moves into function... value borrowed here after move
                        // so s is no longer valid here.

    let x = 5; // x comes in scope

    makes_copy(x); // i32 is `Copy`, so the copy of x's value will moves into function ...
                   // So it's okay to still use x afterward.
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
