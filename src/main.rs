use std::{collections::{HashMap}, vec};

fn main() {
    let s = String::from("hello world");
    let mut a = vec![];
    let mut count = HashMap::new();
    for i in s.chars() {
        a.push(i);
        count.insert(i, 0);
    };
    for j in s.chars() {
        // todo!("if in a than increase count");
        if a.contains(&j) {
            count.entry(j).and_modify(|e| *e += 1);
        }
    };
    println!("{:?}", a);
    println!("{:?}", count);
}
