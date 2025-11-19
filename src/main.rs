fn main() {
    // s is not valid

    let mut s = String::from("hello "); // s is valid and is on the heap

    s.push_str(", World");

    println!("{s}");
} //s is no longer valid
