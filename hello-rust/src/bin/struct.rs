struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("shivasajay"),
        email: String::from("shivasajay007@gmail.com"),
        sign_in_count: 6,
    };
    println!("User 1 Username: {:?}", user1.username);
    println!("User 1 active or not: {:?}", user1.active);
    println!("User 1 email: {:?}", user1.email);
    println!("User 1 sign in count: {:?}", user1.sign_in_count);
}
