// Find area and perimeter of an equilateral triangle
struct Triangle {
    side: f64,
}

struct Rect {
    width: i32,
    height: i32,
}

impl Triangle {
    fn area(&self) -> f64 {
        // For equilateral triangle: area = (√3/4) * side²
        (3_f64.sqrt() / 4.0) * self.side.powi(2)
    }

    fn perimeter(&self) -> f64 {
        // For equilateral triangle: perimeter = 3 * side
        3.0 * self.side
    }
}

impl Rect {
    fn area(&self) -> i32 {
        self.height * self.width
    }
    fn perimeter(&self) -> i32 {
        2 * (self.height + self.width)
    }
}

fn main() {
    let triangle1 = Triangle { side: 4.0 };
    let rect1 = Rect {
        width: 5,
        height: 10,
    };
    // triangle
    println!("Area is: {:.2}", triangle1.area());
    println!("Perimeter is: {:.2}", triangle1.perimeter());

    //rectnagle
    println!("Area is: {:.2}", rect1.area());
    println!("Perimeter is: {:.2}", rect1.perimeter());
}
