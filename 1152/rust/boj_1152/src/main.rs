use std::{io::stdin, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;

    println!("{}", buf.split_whitespace().count());

    Ok(())
}
