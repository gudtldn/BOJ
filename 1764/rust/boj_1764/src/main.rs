use std::{collections::HashSet, error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;

    let (n, m) = {
        let mut temp = buf.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap());
        (temp.next().unwrap(), temp.next().unwrap())
    };

    let mut n_set = HashSet::new();
    for _ in 0..n {
        let mut buf = String::new();
        stdin().read_line(&mut buf)?;

        n_set.insert(buf.trim().to_string());
    }

    let mut m_set = HashSet::new();
    for _ in 0..m {
        let mut buf = String::new();
        stdin().read_line(&mut buf)?;

        m_set.insert(buf.trim().to_string());
    }

    let mut nm_intersection = n_set.intersection(&m_set).collect::<Vec<_>>();
    nm_intersection.sort();

    println!("{}", nm_intersection.len());
    for i in nm_intersection {
        println!("{i}");
    }
    
    Ok(())
}
