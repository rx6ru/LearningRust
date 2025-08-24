fn larger<'a>(a:&'a str, b: &'a str) -> &'a str{
    if a.len() > b.len(){
        a
    }else{
        b
    }
}

fn main(){
    let s1 = String::from("hello world");
    let result;
    // {
    //     let s2 = String::from("hi there");
    //     result = larger(&s1, &s2);
    // } it will show the error because s2 is out of scope
    let s2=String::from("hi there");
    result = larger(&s1, &s2);
    println!("{}", result);
}