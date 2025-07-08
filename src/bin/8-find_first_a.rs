fn main() {
    let s = String::from("holla mami");
    let index = find_first_a(s);

    match index {
        Some(value) => println!("The first 'a' is at index {}", value),
        None => println!("There is no 'a' in the string"),
    }
}
fn find_first_a(s: String) -> Option<u32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as u32);
        }
    }

    return None;
}
