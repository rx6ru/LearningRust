fn main () {
    let mut s1 = String::from("hello world");
    print_str(&s1);
    add_to_it( &mut s1);
    print!("{}", s1);
}

fn print_str(s2: &String) {
    println!("{}", s2);
}

fn add_to_it(s3: &mut String) {
    s3.push_str("!");
}