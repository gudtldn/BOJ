use std::{error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    let _ = input::<usize>();
    let order: Vec<usize> = input::<String>()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut result = vec![];
    for (n, &num) in order.iter().enumerate() {
        result.insert(num, n + 1);
    }

    for i in result.iter().rev() {
        print!("{} ", i);
    }

    Ok(())
}

fn input<T>() -> T
where
    T: std::str::FromStr,
    T::Err: Error,
{
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
