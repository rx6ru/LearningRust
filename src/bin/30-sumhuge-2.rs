use std::{
    io::{self, Write}, sync::mpsc, thread
};

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut input = String::new();
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let res=input.trim().parse();
    let num:u64;
    match res{
        Ok(val)=> num=val,
        Err(e)=> panic!("{}",e),
    };

    let chunk=num/8;
    for i in 0..8{
        let producer =tx.clone();

        thread::spawn(move || {
            let mut ssum: u64=0;
            for j in (i*chunk+1)..=(i+1)*chunk{
                ssum+=j;
            }
            let res=producer.send(ssum);
            match res{
                Ok(_)=> {},
                Err(e)=> println!("{}",e),
            }
        });
    } 

    drop(tx);

    let mut sum=0;
    for i in rx{
        sum+=i;
    }
    println!("{}",sum); 
}