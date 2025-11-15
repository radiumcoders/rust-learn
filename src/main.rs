fn main() {
    println!("printing from main");
    another_function(23);
    st_ex();
}

//statements and expressions :2025-11-15 14:24

fn st_ex() {
    let x = 23; //statement :D 
    let y = 34;
    let z = x + y; // expressions
    println!("statement {x} , {y} and expressions {z}")
}

// function with arguments  :2025-11-15
fn another_function(num: i32) {
    println!("printing from another_function with argument of {num}");
}

//simple example :11/15/2025
//fn another_function() {
//  println!("printing from another_function")
//}
