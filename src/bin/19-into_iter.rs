fn main() {
    let vec=vec![1,2,3,4,5];
    let itr=vec.into_iter();

    for i in itr {
        println!("Got: {i}")
    }
}
