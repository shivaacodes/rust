enum Direction {
    North,
    South,
    East,
    West,
}

enum Shape {
    Square(f64), // variant with associated data (side_length)
    Circle(f64), // variant with associated data (radius)
}

fn main() {
    let my_direction = Direction::North;
    let my_shape = Shape::Circle(5.0);

    move_around(my_direction);
    print_area(my_shape);
}

fn move_around(direction: Direction) {
    let direction_str = match direction {
        Direction::North => "North",
        Direction::South => "South",
        Direction::East => "East",
        Direction::West => "West",
    };
    println!("Direction is: {}", direction_str);
}

fn print_area(shape: Shape) {
    let area = match shape {
        Shape::Square(side_length) => side_length * side_length,
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
    };
    println!("Area is: {:.2}", area);
}
