trait Sound {
    fn sound(&self);
}

struct Dog;
struct Cat;

impl Sound for Dog {
    fn sound(&self) {
        println!("woof");
    }
}

impl Sound for Cat {
    fn sound(&self) {
        println!("meow");
    }
}

fn main() {
    let dog = Dog;
    let cat = Cat;
    dog.sound();
    cat.sound();
}