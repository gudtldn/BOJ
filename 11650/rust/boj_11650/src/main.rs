use std::{error::Error, io::stdin, fmt::Write};

fn main() -> Result<(), Box<dyn Error>> {
    let n = {
        let mut buf = String::new();
        stdin().read_line(&mut buf)?;
        buf.trim().parse::<usize>()?
    };

    let mut points: Vec<(i32, i32)> = (0..n)
        .map(|_| {
            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            let mut iter = buf.trim().split_whitespace();
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    points.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let mut buf = String::new();
    for (x, y) in points {
        write!(buf, "{} {}\n", x, y)?;
    }

    print!("{}", buf);

    Ok(())
}
