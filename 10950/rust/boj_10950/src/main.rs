use std::{error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;

    for _ in 0..buf.trim().parse::<usize>().unwrap() {
        buf.clear();
        stdin().read_line(&mut buf)?;

        println!(
            "{}",
            buf.trim()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .sum::<u32>()
        );
    }

    Ok(())
}
