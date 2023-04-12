use std::{io::stdin, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;

    let n = buf.trim().parse::<usize>().unwrap();
    let (mut a, mut b) = (0_u32, 1_u32);
    for _ in 1..=n {
        let temp_a = a;
        a = b;
        b += temp_a;
    }

    println!("{} {}", a, n-2);
    
    Ok(())
}
