fn main() {
    let s = String::from("Hello world!");

    let s_ref1 = &s[..5];
    let s_ref2 = &s[6..];

    println!(
        "s_ref1 is `{}`, and it's length is {}",
        s_ref1,
        s_ref1.len()
    );
    println!(
        "s_ref2 is `{}`, and it's length is {}",
        s_ref2,
        s_ref2.len()
    );
}
