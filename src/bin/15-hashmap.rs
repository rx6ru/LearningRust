use std::collections::HashMap;

fn main() {
    let mut users=HashMap::new();
    users.insert(String::from("Amit"),95);
    users.insert( String::from("Rahul"),64);
    let user_score=users.get("Amit");
    match user_score{
        Some(value)=>println!("Amit's score is {}",value),
        None=>println!("User not found"),
    }
}