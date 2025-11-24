struct User {
    name: String,
    email: String,
    age: u32,
    is_active: bool,
}

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
}
