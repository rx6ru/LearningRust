use std::{
   sync::mpsc,
   thread::spawn
};
fn main() {
    let (tx, rx) = mpsc::channel();
    spawn(move || {
        let val=String::from("Hi");
        let x=tx.send(val);
        match x {
            Ok(_) => println!("Message sent successfully!"),
            Err(e) => println!("Error sending message: {}", e),
        }
    });

    let received = rx.recv();
    match received{
      Ok(msg)=> println!("Message received: {}", msg),
      Err(e) => println!("Error receiving message: {}", e),
    }
}
