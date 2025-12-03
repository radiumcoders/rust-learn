// #[derive(Debug)]
// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }

// #[derive(Debug)]
// struct Home {
//     address: String,
//     direction:Direction,
// }

// fn main () {
//     let up = Direction::Up;
//     println!("{:?}", up);
//     let home = Home  {
//         address: String::from("1234, marconi street"),
//         direction: Direction::Up
//     };
//     println!("{:?}", home);
// }
#[derive(Debug)]
enum Methods{
    Create(String),
    Locate(i32),
    Quit
}

impl Methods {
    fn call (&self) {
        println!("{:?}",self)
    }
}


fn main () {
    let g = Methods::Create(String::from("Hello"));
    g.call();
}