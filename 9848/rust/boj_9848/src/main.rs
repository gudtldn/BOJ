use std::{error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_buffer = String::new();
    stdin().read_line(&mut input_buffer)?;

    let (n, k) = {
        let mut iter = input_buffer.split_whitespace();
        let n: i32 = iter.next().unwrap().parse()?;
        let k: i32 = iter.next().unwrap().parse()?;
        (n, k)
    };

    let mut prev_value = 0;
    let mut total_gifts = 0;
    for i in 0..n {
        input_buffer.clear();
        stdin().read_line(&mut input_buffer)?;
        let current_value = input_buffer.trim().parse::<i32>()?;

        if i > 0 {
            total_gifts += if (prev_value - current_value) >= k {
                1
            } else {
                0
            };
            prev_value = current_value;
        } else {
            prev_value = current_value;
        }
    }

    println!("{total_gifts}");
    Ok(())
}
