fn main() {
    println!("{}", fib_with_recursion(10));
    println!("{}", fib_with_loop(10));
}

fn fib_with_loop(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    if n <= 1 {
        return n;
    }
    for _ in 0..(n - 1) {
        let c = a + b;
        a = b;
        b = c;
    }
    return b;
}

fn fib_with_recursion(n: i32) -> i32 {
    if n > 1 {
        fib_with_recursion(n - 1) + fib_with_recursion(n - 2)
    } else {
        return n;
    }
}
