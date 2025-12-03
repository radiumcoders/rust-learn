#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Home {
    address: String,
    direction:Direction,
}

fn main () {
    let up = Direction::Up;
    println!("{:?}", up);
    let home = Home  {
        address: String::from("1234, marconi street"),
        direction: Direction::Up
    };
    println!("{:?}", home);
}