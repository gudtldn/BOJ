use std::{error::Error, io::stdin};

struct Change {
    quarter: u32,
    dime: u32,
    nickel: u32,
    penny: u32,
}

impl Change {
    fn new() -> Change {
        Change {
            quarter: 0,
            dime: 0,
            nickel: 0,
            penny: 0,
        }
    }
}

impl std::fmt::Display for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.quarter, self.dime, self.nickel, self.penny
        )
    }
}

fn divmod<T>(a: T, b: T) -> (T, T)
where
    T: std::ops::Div<Output = T> + std::ops::Rem<Output = T> + Copy,
{
    (a / b, a % b)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut n = String::new();
    stdin().read_line(&mut n)?;

    for _ in 0..n.trim().parse::<usize>().unwrap() {
        let mut cent = String::new();
        stdin().read_line(&mut cent)?;

        let mut cent = cent.trim().parse::<u32>()?;
        let mut change = Change::new();

        (change.quarter, cent) = divmod(cent, 25);
        (change.dime, cent) = divmod(cent, 10);
        (change.nickel, cent) = divmod(cent, 5);
        change.penny = cent;

        println!("{change}");
    }

    Ok(())
}
