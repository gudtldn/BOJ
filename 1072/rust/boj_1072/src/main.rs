use std::{error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;

    let (x, y) = {
        let mut temp = buf
            .trim()
            .split_whitespace()
            .map(|i| i.parse::<u64>().unwrap());
        (temp.next().unwrap(), temp.next().unwrap())
    };

    let z = (y * 100) / x;

    if z >= 99 {
        println!("-1");
        return Ok(());
    }

    let mut low = 0_u64;
    let mut high = 1_000_000_000;

    let mut res = 0;

    while low <= high {
        let mid = (low + high) / 2;
        let temp_z = ((y + mid) * 100) / (x + mid);

        if z >= temp_z {
            res = mid + 1;
            low = mid + 1;
        } else {
            high = mid -1
        }
    }

    println!("{res}");
    
    Ok(())
}
