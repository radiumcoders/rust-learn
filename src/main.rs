fn main() {
    println!("control flow :D ");
    if_else();
    let_if();
    nes_if();
    ops();
    loopalopi();
    whilee();
}

//basic if else statement
fn if_else() {
    let x = 23;
    if x > 30 {
        print!("x is greater than 30");
    } else {
        println!("x is smaller than 30");
    };
}

//if with let statement
fn let_if() {
    let condition = false;
    let x = if condition { 5 } else { 10 };
    println!("{x}")
}

// nested if condition
fn nes_if() {
    let num = 12;
    if num % 2 == 0 {
        println!("num is even");
    } else {
        println!("num is odd");
        if num >= 10 {
            println!("num is larger or equal to 10");
        } else {
            println!("num is smaller than 10")
        }
    }
}

// && and \\ operators :D
fn ops() {
    let (a, b, c) = (5, 20, 10);
    if a < b && a < c {
        println!("a is smaller than b and c both")
    };
    if b > a || c > b {
        println!("b is larger than a or c is larger than b ")
    }
}

//loop keyword
fn loopalopi() {
    // loop{} ---->> infinite loop
    // returning value through loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        };
    };
    println!("{result} , {counter}")
}

//while loop
fn whilee() {
    let mut var = 0;
    while var <= 10 {
        println!("{var}");
        var += 1;
    }
}

//for loop
