fn main() {
    println!("printing from main");
    another_function(23);
    st_ex();
    let x = returner();
    println!("the value returned from func is ==> {x}");
}

//statements and expressions :2025-11-15 14:24

fn st_ex() {
    // let x = 23; //statement :D
    // let y = 34;
    // let z = x + y; // expressions
    // println!("statement {x} , {y} and expressions {z}")

    let y = {
        let x = 23;
        x + 12 // here semicolon will give an error :| 
    };
    println!("{}", y);
}

// function with arguments  :2025-11-15
fn another_function(num: i32) {
    println!("printing from another_function with argument of {num}");
}

//simple example :11/15/2025
//fn another_function() {
//  println!("printing from another_function")
//}

//return values :11/15/2025

fn returner() -> i32 {
    println!("printing from the returning function :D");
    1 + 2
}
