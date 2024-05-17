use std::{error::Error, io::stdin};

fn input<T>() -> T
where
    T: std::str::FromStr,
    T::Err: Error,
{
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse::<T>().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut heights: Vec<u32> = (0..9).map(|_| input()).collect();
    let sum = heights.iter().sum::<u32>();

    'outer: for i in 0..9 {
        for j in i + 1..9 {
            if sum - heights[i] - heights[j] == 100 {
                heights.remove(i);
                heights.remove(if i < j { j - 1 } else { j });
                heights.sort();
                break 'outer;
            }
        }
    }

    for height in heights {
        println!("{}", height);
    }

    Ok(())
}
