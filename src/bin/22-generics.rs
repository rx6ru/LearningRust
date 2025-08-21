fn main() {
    let res=bigger('1','2');
    let res2=bigger(1,2);
    println!("{} {}",res, res2);  
    let num=std::f32::NAN;
    let res3=bigger(num,2.0);
    println!("{}",res3);
} 


fn bigger<T:std::cmp::PartialOrd>(x: T, y: T) -> T {
    if x > y {
        x
    } else {  
        y
    }
}