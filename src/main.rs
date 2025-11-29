struct User {
    name: String,
    email: String,
    age: u32,
    is_active: bool,
}

struct Color(i32, i32, i32);

#[derive(Debug)]
struct Unit;

fn main() {
    let user1 = User {
        name: String::from("Radium COders"),
        email: String::from("radiumcoders@gmail.com"),
        age: 17,
        is_active: true,
    };
    // println!(
    //     "User Details \n Name :- {} \n Email :- {} \n Age :- {} \n IsActive :- {}",
    //     user1.name, user1.email, user1.age, user1.is_active
    // )
    println!(
        r#"User Details
    Name :- {}
    Email :- {}
    Age :- {}
    IsActive :- {}"#,
        user1.name, user1.email, user1.age, user1.is_active
    );

    let user2 = User {
        name: String::from("Jay sharma"),
        email: user1.email,
        age: user1.age,
        is_active: false,
    };

    println!(
        r#"User Details
    Name :- {}
    Email :- {}
    Age :- {}
    IsActive :- {}"#,
        user2.name, user2.email, user2.age, user2.is_active
    );
    // this will give error as the value is moved to user2 and mow can not be borrowed
    // :X
    // println!(
    //     r#"User Details
    // Name :- {}
    // Email :- {}
    // Age :- {}
    // IsActive :- {}"#,
    //     user1.name, user1.email, user1.age, user1.is_active
    // );

    //tupple structs
    let tup_str = Color(1212, 0000, 1234);
    println!(
        "values from tupple struct 1st {} 2nd {} 3rd {}",
        tup_str.0, tup_str.1, tup_str.2
    );

    let uni_str = Unit;
    println!("{:?}", uni_str);

    //struct functions are comming
}
