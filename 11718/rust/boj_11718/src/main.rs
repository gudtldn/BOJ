use std::{io::{stdin, Read}, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf)?;

    println!("{buf}");
    
    Ok(())
}
