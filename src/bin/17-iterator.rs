fn main () {
    let mut v1=vec![1,2,3];
    let iterator=v1.iter_mut();

    for i in iterator {
        *i+=1;
        println!("Got: {i}");
    }
}