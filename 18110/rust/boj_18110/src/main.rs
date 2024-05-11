use std::{error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    let mut difficulty: u32 = 0;

    let n: usize = input_to_numberic();
    if n == 0 {
        println!("{difficulty}");
        return Ok(());
    }

    let mut ratings: Vec<u32> = (0..n).map(|_| input_to_numberic()).collect();
    ratings.sort();

    let start = (n as f64 * 0.15).round() as usize;
    let trimmed = &ratings[start..n-start];
    let trimmed_sum: u32 = trimmed.iter().sum();

    difficulty = (trimmed_sum as f64 / trimmed.len() as f64).round() as u32;

    println!("{difficulty}");
    Ok(())
}

fn input_to_numberic<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse().unwrap()
}
