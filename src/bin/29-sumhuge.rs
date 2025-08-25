//using thread

use std::thread;

fn main(){
    let n=100000000;
    let no_threads=8;
    let chunk_size=n/no_threads;
    let mut handles=vec![];

    for i in 0..no_threads{
        let handle=thread::spawn( move || {
            let mut ssum: u64=0;

            for j in (i*chunk_size+1)..(i+1)*chunk_size+1{
                ssum+=j;
            }
            ssum
        });
        handles.push(handle);
    }

    let mut sum=0;
    for i in handles{
        let x=i.join();
        match x{
            Ok(val)=>sum+=val,
            Err(e)=>println!("{:?}",e),
        }
    }
    println!("{}",sum);
}