fn main(){
    let name=String::from("Amar Mahato");
    let fword=first_word(&name);
    println!("{}",fword);
}

fn first_word(s: &String ) -> &str{
    let mut index=0;

    for i in s.chars(){
        if i==' '{
            break;
        }
        index+=1;
    }  
    &s[0..index]
}