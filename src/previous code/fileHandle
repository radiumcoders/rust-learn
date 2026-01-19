use std::{fs::File, io::Read};

// fn main() {
//     // let file_opened = File::open("hello.txt").expect("hello");
//     let file_opened = File::open("hello.txt");
//     match file_opened {
//         Ok(file) => {
//             println!("File opened successfully: {:?}", file);
//         }
//         Err(e) => {
//             println!("Failed to open the file creating a new one. Error: {:?}", e);
//             File::create("hello.txt")
//                 .unwrap_or_else(|error| panic!(" error creating file :( error -< {}", error));
//         }
//     }
// }

//reading username from a file :)

fn read_user_name() -> Result<String, std::io::Error> {
    let f = File::open("hey.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => panic!("Problem opening the file: {:?}", e)
    };
    let mut s = String::new();
    match f.read_to_string(&mut s){
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn main() {
    match read_user_name() {
        Ok(name) => println!("Username: {}", name),
        Err(e) => println!("Error reading username: {:?}", e),
    }
}