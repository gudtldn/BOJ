use std::{io::stdin, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf)?;

        if buf.trim() == "0" {
            break;
        }

        let n = buf.trim().parse::<u32>()?;
    }

    Ok(())
}
