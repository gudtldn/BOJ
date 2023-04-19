use std::{collections::HashMap, error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    let mut map: HashMap<i64, i64> = HashMap::new();

    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    buf.clear();

    stdin().read_line(&mut buf)?;
    for num in buf
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
    {
        *map.entry(num).or_insert(0) += 1;
    }
    buf.clear();

    stdin().read_line(&mut buf)?;
    buf.clear();

    stdin().read_line(&mut buf)?;
    let b = buf.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap());

    for num in b {
        match map.get(&num) {
            Some(x) => print!("{} ", x),
            None => print!("0 "),
        }
    }

    Ok(())
}
