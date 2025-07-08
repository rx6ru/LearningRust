struct User {
    first_name: String,
    last_name: String,
    age: u8,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        first_name: String::from("Amit"),
        last_name: String::from("Chaudhary"),
        age: 30,
        email: String::from("BwB0D@example.com"),
        sign_in_count: 2,
        active: true,
    };

    println!("{} {}", user1.first_name, user1.last_name);
    println!("Age: {}", user1.age);
    println!("Email: {}", user1.email);
    println!("Sign in count: {}", user1.sign_in_count);
    println!("Active: {}", user1.active);
}
