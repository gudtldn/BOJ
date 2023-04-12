use std::{error::Error, io::stdin};

trait IsSorted {
    fn _is_sorted(&self, reverse: bool) -> bool;
}

impl IsSorted for Vec<u32> {
    fn _is_sorted(&self, reverse: bool) -> bool {
        for idx in 1..self.len() {
            if self[idx - 1] <= self[idx] && reverse {
                return false;
            } else if self[idx - 1] >= self[idx] && !reverse {
                return false;
            }
        }
        true
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;

    let numbers = buf
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    if numbers._is_sorted(false) {
        println!("ascending");
    } else if numbers._is_sorted(true) {
        println!("descending");
    } else {
        println!("mixed");
    }

    Ok(())
}
