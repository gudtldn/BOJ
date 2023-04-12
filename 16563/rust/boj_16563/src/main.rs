use std::{
    error::Error,
    io::{self, BufRead, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let _ = lines.next();
    let k_vec = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let stdout = io::stdout().lock();
    let mut out = io::BufWriter::new(stdout);

    for mut k in k_vec {
        let mut vector = Vec::new();

        for i in 2u64..=((k as f64).sqrt() as u64) + 1 {
            while k % i == 0 {
                vector.push(i);
                k /= i;
            }
        }

        if k > 1 {
            vector.push(k);
        }

        vector.sort();
        write!(
            out,
            "{}\n",
            vector
                .iter()
                .map(|x| x.to_string())
                .fold(String::new(), |mut acc, x| {
                    acc.push_str(&x);
                    acc.push(' ');
                    acc
                })
        )?;
    }

    Ok(())
}
