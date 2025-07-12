use std::collections::HashMap;

fn v2hm(v: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for (key, value) in v {
        map.insert(key, value);
    }
    map
}

fn main() {
    let pairs = vec![
        (String::from("Amit"), 95),
        (String::from("Rahul"), 64),
        (String::from("Rohit"), 85),
        (String::from("Sachin"), 90),
    ];

    let result = v2hm(pairs);
    println!("{:?}", result);
}
