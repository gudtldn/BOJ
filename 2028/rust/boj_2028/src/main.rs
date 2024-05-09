use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut t = String::new();
    io::stdin().read_line(&mut t)?;

    for _ in 0..t.trim().parse::<i32>()? {
        let mut n = String::new();
        io::stdin().read_line(&mut n)?;

        let n_square = n.trim().parse::<i64>()?.pow(2).to_string();
        println!("{}", if n_square.ends_with(&n.trim()) { "YES" } else { "NO" });
    }

    Ok(())
}
