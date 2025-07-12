fn main () {
    let mut vec1=vec![1,2,3,4,5];
    let mut iterator=vec1.iter_mut();

    while let Some(val)=iterator.next() {
        *val+=1;
        println!("Got: {val}")
    }
}