fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result = filter_even(vec);
    println!("{:?}", result);
}

fn filter_even(vec: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in vec {
        if i % 2 == 0 {
            result.push(i);
        }
    }
    result
}
