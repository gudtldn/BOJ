use std::{error::Error, io::stdin};

fn main() {
    for _ in 0..input_t::<usize>() {
        let mut case: Vec<_> = input()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        case.sort_unstable();
        println!("{}", case[case.len() - 3])
    }
}

fn input_t<T>() -> T
where
    T: std::str::FromStr,
    T::Err: Error,
{
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
