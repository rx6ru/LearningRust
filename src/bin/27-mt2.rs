use std::thread;

fn main(){
    let v=vec![1,2,3,4,5];
    let handle=thread::spawn(move || {
        println!("item is {v:?}");
    });

    handle.join().unwrap();
}