struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = build_user(String::from("abc@mail.com"), String::from("rjun"));

    println!("User1's original email is: {}", user1.email);
    user1.email = String::from("fakemaillll@mail.com");
    println!("After modifying email: {}", user1.email);

    let user2 = User {
        email: String::from("another@mail.com"),
        ..user1 // 剩下沒指名的欄位都會取得 user1 實例相同的值
                // user1 的值會被 move 到 user2
    };
}
