// Time Out

use std::{io::stdin, error::Error, fmt::Display};

struct User {
    age: u8,
    name: String,
    order: usize,
}

impl User {
    fn new(age: u8, name: String, order: usize) -> Self {
        User { age, name, order }
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.age, self.name)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let n: usize = {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;
        buffer.trim().parse()?
    };

    let mut users: Vec<User> = vec![];

    for idx in 0..n {
        let (age, name) = {
            let mut buffer = String::new();
            stdin().read_line(&mut buffer)?;
            let mut iter = buffer.trim().split_whitespace();
            (iter.next().unwrap().parse::<u8>()?, iter.next().unwrap().to_string())
        };
        users.push(User::new(age, name, idx));
    }

    users.sort_by(|a, b| {
        if a.age == b.age {
            a.order.cmp(&b.order)
        } else {
            a.age.cmp(&b.age)
        }
    });

    for user in users {
        println!("{}", user);
    }
    Ok(())
}
