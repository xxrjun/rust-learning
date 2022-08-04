fn main() {
    let s1 = String::from("Hello");

    /*
        references 使我們可以不用交出 ownership。
        建立 reference 的動作叫做 borrowing。
        且 reference 的值預設也是 immutable。
    */
    let len1 = calculate_length(&s1);

    println!("'{}' 的長度為 {}", s1, len1);

    let mut s2 = String::from("This is s2");

    change(&mut s2);

    println!("s2: {}", s2);
}

fn calculate_length(s: &String) -> usize {
    // s is a refernece to a string
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", hello");
}
