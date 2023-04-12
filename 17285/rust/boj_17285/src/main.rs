use std::{error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;

    let encrypted_chars = buf.trim().chars().map(|c| c as u8);
    let mut decrypted_text = String::new();

    for encrypted_char in encrypted_chars {
        decrypted_text += &((encrypted_char ^ 22) as char).to_string();
    }

    println!("{decrypted_text}");

    Ok(())
}
