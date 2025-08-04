struct Rect {
    width: i32,
    height: i32,
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
    let rect1 = Rect {
        width: 8,
        height: 12,
    };

    println!("Area is: {}", rect1.area());
    println!("Perimeter is: {}", rect1.perimeter());
}
