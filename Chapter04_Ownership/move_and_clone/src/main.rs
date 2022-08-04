fn main() {
    /*
        1. Move:
            It like shallow copy in other programming langs.
            But rust will also invalidates the first variable.
            So we call it `Move`
    */

    // In integer
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    println!();

    // In String
    let s1 = String::from("Hello");

    /*
        將 s1 賦值給 s2，stack 上的指標、長度與容量會被拷貝。指標指向的堆積資料不會被拷貝，否則 cost 會很大。
    */
    let s2 = s1; // s1 is no longer valid after this line.

    println!("s2: {}", s2);
    println!();

    // ------------------------------------------------

    /*
        2. Clone: like deep copy in other programming langs.
    */
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    println!();

    /*
        3. Stack-Only Data: Copy

        Here are some of the types that implement `Copy`:
            - All integer types
            - bool
            - All float types
            - char
            - tuple, if they only contain types that also implement `Copy`
    */
}
