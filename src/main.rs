fn main() {
    let i = 5;
    let s = String::from("ruster");
    call_int(i);
    println!("after calling fn to print value -> {i}");
    call_str(s);
    //this will give error why on day 10 XD
    //println!("after calling fm to print value => {s}");
}

fn call_int(i: i32) {
    println!("printing from fn {i}");
}
fn call_str(s: String) {
    println!("printing from fn {s}");
}

//fn main() {
//  let mut i = 5;
//caller_changer(&mut i);
//println!("the value of i after {i}")
//}

//fn caller_changer(i: &mut i32) {
//  println!("the value of i recived to the fn is {i}");
//*i = 23;
//}
