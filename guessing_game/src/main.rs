use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("請猜測一個數字!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("請輸入你的猜測數字: ");

    let mut guess = String::new();  // mut 可變的

    io::stdin()
        .read_line(&mut guess)
        .expect("讀取該行失敗");
    
    println!("你的猜測數字: {}", guess);
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("太小了!"),
        Ordering::Greater => println!("太大!"),
        Ordering::Equal => println!("Bingo!"),
    }
}
