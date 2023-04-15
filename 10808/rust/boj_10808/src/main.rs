use std::{io::stdin, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let mut alphabet = [0; 26];
    
    let mut s = String::new();
    stdin().read_line(&mut s)?;

    for c in s.trim().chars() {
        alphabet[(c as u8 - 'a' as u8) as usize] += 1;
    }

    println!("{}", alphabet.map(|x| x.to_string()).join(" "));
    
    Ok(())
}
