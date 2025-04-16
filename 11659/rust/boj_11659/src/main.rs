// https://www.acmicpc.net/problem/11659

use std::str::FromStr;

fn parse_line<T>(next_str: &str) -> (T, T)
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut iter = next_str.split_whitespace().map(|x| x.parse::<T>().unwrap());
    (iter.next().unwrap(), iter.next().unwrap())
}

fn solution(stdin: &str) {
    let mut tokens = stdin.split('\n');
    let mut next = || tokens.next().unwrap();
    let (_, m) = parse_line(next());

    let sum_vec = std::iter::once(0)
        .chain(
            next()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .scan(0, |acc, x| {
                    *acc += x;
                    Some(*acc)
                })
        )
        .collect::<Vec<_>>();

    (0..m).for_each(|_| {
        let (i, j) = parse_line::<usize>(next());
        println!("{}", sum_vec[j] - sum_vec[i - 1]);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    fn capture_output(f: impl FnOnce()) -> String {
        STDOUT.with(|stdout| {
            stdout.borrow_mut().clear();
        });
        f();
        STDOUT.with(|stdout| {
            String::from_utf8(stdout.borrow().to_vec())
                .unwrap()
                .trim_end()
                .to_string()
        })
    }

    #[test]
    fn test_solution1() {
        assert_eq!(
            capture_output(|| solution("5 3\n5 4 3 2 1\n1 3\n2 4\n5 5")),
            "12\n9\n1"
        );
    }
}

fn main() {
    use std::io::*;
    solution(&read_to_string(stdin()).unwrap());
    STDOUT.with(|refcell| std::io::Write::flush(&mut *refcell.borrow_mut()).unwrap());
}

#[cfg(test)]
thread_local! {
    static STDOUT: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(Vec::new());
}

#[cfg(not(test))]
thread_local! {
    static STDOUT: std::cell::RefCell<std::io::BufWriter<std::io::StdoutLock<'static>>> =
    std::cell::RefCell::new(std::io::BufWriter::with_capacity(1 << 17, std::io::stdout().lock()));
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
