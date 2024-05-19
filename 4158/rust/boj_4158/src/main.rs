use std::{collections::HashSet, error::Error};

fn solve(stdin: &str) -> Result<(), Box<dyn Error>> {
    let mut tokens = stdin.lines();
    let mut next = || tokens.next().unwrap();
    loop {
        let (n, m) = {
            let mut iter = next().split_whitespace();
            (
                iter.next().unwrap().parse::<usize>()?,
                iter.next().unwrap().parse::<usize>()?,
            )
        };

        if n == 0 && m == 0 {
            break;
        }

        let mut n_set: HashSet<u64> = HashSet::new();

        for _ in 0..n {
            n_set.insert(next().trim().parse::<u64>()?);
        }

        let mut count = 0u64;
        for _ in 0..m {
            let num = next().trim().parse::<u64>()?;
            if n_set.contains(&num) {
                count += 1;
            }
        }

        println!("{count}");
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    use std::io::*;
    solve(&read_to_string(stdin()).unwrap())?;
    STDOUT.with(|refcell| std::io::Write::flush(&mut *refcell.borrow_mut()).unwrap());
    Ok(())
}

thread_local! {
    static STDOUT: std::cell::RefCell<std::io::BufWriter<std::io::StdoutLock<'static>>> = std::cell::RefCell::new(std::io::BufWriter::with_capacity(1 << 17, std::io::stdout().lock()));
}

#[macro_export]
macro_rules! println {
    ($($t:tt)*) => {
        STDOUT.with(|refcell| {
            use std::io::*;
            writeln!(refcell.borrow_mut(), $($t)*).unwrap();
        });
    };
}
#[macro_export]
macro_rules! print {
    ($($t:tt)*) => {
        STDOUT.with(|refcell| {
            use std::io::*;
            write!(refcell.borrow_mut(), $($t)*).unwrap();
        });
    };
}
