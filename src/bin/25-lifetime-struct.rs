struct User<'a>{
    name:&'a str
}

fn main(){
    let f_name=String::from("Amar");
    let user=User{name:&f_name}; 
    println!("{}",user.name);
} 