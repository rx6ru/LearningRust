use std::fs::read_to_string;

fn main(){
    let result=read_to_string("src/hello.txt");
    match result {
        Ok(value) => println!("{}", value),
        Err(error) => println!("{}", error),
    }
    
}