fn main() {
    /*
        When we call `String::from`, memory must be requested from the memory
        allocator at runtime
    */
    let mut s = String::from("hello"); // s is valid from this point forward

    s.push_str(", world!");

    println!("{}", s);

    /*
        Rust 沒有 GC，而是當記憶體在擁有它的變數離開作用域 (scope) 時就會自動釋放
    */
} // this scope is now over, and s is no longer valid
  // Rust will call a special funtion `drop()`
