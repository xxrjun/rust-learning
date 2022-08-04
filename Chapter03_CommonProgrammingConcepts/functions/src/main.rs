fn main() {
    another_function();

    let x = 5;
    let y = 6;
    let result = plus_function(x, y);

    println!("{} + {} = {}", x, y, result);
}

fn another_function() {
    println!("This is another function!");
}

fn plus_function(x: i32, y: i32) -> i32 {
    println!("This is a plus function.");
    return x + y;
}
