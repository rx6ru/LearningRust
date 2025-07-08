enum Shape{
    Rectangle(f64, f64),
    Circle(f64),
}

fn main(){
    let rect = Shape::Rectangle(10.3, 20.0);
    let circle = Shape::Circle(15.6);

    println!("The area of the rectangle is {} square pixels.", area(rect));
    println!("The area of the circle is {} square pixels.", area(circle));
}

fn area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(width, height) => width * height,
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius
    }
}
