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
// #[derive(Debug)]
// enum Methods{
//     Create(String),
//     Locate(i32),
//     Quit
// }
 
// impl Methods {
//     fn call (&self) {
//         println!("{:?}",self)
//     }
// }


// fn main () {
//     let g = Methods::Create(String::from("Hello"));
//     g.call();
// }



fn main ()  {
    let string_option_some = Some(String::from("Hello Rust"));
    let string_option_none : Option<String> = None;
    match string_option_some {
        Some(value) => println!("Some({})", value),
        None => println!("None"),
    }
    match string_option_none {
        Some(value) => println!("Some({})", value),
        None => println!("None"),
    }
}