use std::{error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    let dial = ["ABC", "DEF", "GHI", "JKL", "MNO", "PQRS", "TUV", "WXYZ"].iter().map(|x| x.chars());

    let mut buf = String::new();
    stdin().read_line(&mut buf)?;

    let mut total_time = 0;

    for c in buf.trim().chars() {
        for (n, mut d) in dial.clone().enumerate() {
            if d.any(|x| c == x) {
                total_time += n as u32 + 3;
                break;
            }
        }
    }
    
    println!("{total_time}");
    
    Ok(())
}
