use std::thread;
// use std::time::Duration;

fn main(){
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread 1 !", i);
            // thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..10 {
        println!("hi number {} from the main thread!", i);
        // thread::sleep(Duration::from_millis(100));
    }
}