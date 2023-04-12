use std::{error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;

    println!(
        "{}",
        buf.trim()
            .split_whitespace()
            .map(|x| x.chars().rev().collect::<String>().parse::<u32>().unwrap())
            .max()
            .unwrap()
    );

    Ok(())
}
