use chrono::{Local,Utc};

fn main(){
    println!("Local: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
    println!("UTC: {}", Utc::now().format("%Y-%m-%d %H:%M:%S"));
}