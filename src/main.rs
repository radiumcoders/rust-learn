//traits we are going to learn :)
#[derive(Debug)]
struct Tweet {
    username: String,
    body: String,
    on: u32,
}

trait Summary {
    fn summary(&mut self) -> String;
}

impl Summary for Tweet {
    fn summary(&mut self) -> String {
        self.on += 1;
        format!("{}: {}", self.username, self.body)
    }
}

fn main() {
    let mut teww = Tweet {
        username: String::from("RadiumCoders"),
        body: String::from("Learning Rust traits is fun!"),
        on: 0,
    };
    println!("Tweet Summary: {}", teww.summary());
}
