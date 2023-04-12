use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        if buf.trim().is_empty() {
            return Ok(());
        }

        let (mut n, k) = {
            let mut split = buf.trim().split(" ");
            let n = split.next().unwrap().parse::<u64>()?;
            let k = split.next().unwrap().parse::<u64>()?;
            (n, k)
        };

        let mut x = n;

        while n / k != 0 {
            x += n / k;
            n = n / k + n % k;
        }

        println!("{x}");
    }
}
