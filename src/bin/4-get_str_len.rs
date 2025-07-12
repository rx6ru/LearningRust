fn main() {
    let str = String::from("hi Alien");
    println!("Length = {}", get_str_len(&str));
}

fn get_str_len(s: &str) -> usize {
    s.chars().count()
}
