use std::{io::stdin, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut x = String::new();
    stdin().read_line(&mut x)?;
    
    let mut n = String::new();
    stdin().read_line(&mut n)?;

    let mut total = 0u32;

    for _ in 0..n.trim().parse().unwrap() {
        let mut buf = String::new();
        stdin().read_line(&mut buf)?;

        let (a, b) = {
            let mut buf = buf.trim().split_whitespace().map(|x| x.parse::<u32>().unwrap());
            (buf.next().unwrap(), buf.next().unwrap())
        };

        total += a*b;
    }

    println!("{}", if x.trim().parse::<u32>().unwrap() == total { "Yes" } else { "No" });


    Ok(())
}
