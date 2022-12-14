trait Shape {
    fn area(&self) -> f64;
}

fn print_area<T: Shape>(shape: T) {
    println!("The area of the shape is: {}", shape.area());
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
}

fn main() {
    let circle = Circle { radius: 2.0 };
    let triangle = Triangle { base: 3.0, height: 4.0 };

    print_area(circle);
    print_area(triangle);
}
