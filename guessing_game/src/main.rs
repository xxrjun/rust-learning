use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("請猜測一個數字!");

    /*
       1. Using rand crate to generate a random secret number.
       2. A crate is a collection of Rust source code files.
       3. gen_range(start..end)
          gen_range(1..=100) is equal to .gen_range
    */
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("請輸入你的猜測數字: ");

        let mut guess = String::new(); // mut: mutable
        io::stdin().read_line(&mut guess).expect("讀取該行失敗"); // Handle error

        /*
            1. i32: a 32-bits number, u32: a 32-bits unsigned number
            2. trim(): eliminate any whitespace at the beggining and end.
            3. parse(): converts a String to another type.
        */
        let guess: u32 = match guess.trim().parse() {
            // Arms and Patterns
            Ok(num) => num,
            Err(_) => continue, // _ is a catchall value
        };
        println!("你的猜測數字: {}", guess);

        // Compare guess number and secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了!"),
            Ordering::Greater => println!("太大!"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            }
        }
    }
}
