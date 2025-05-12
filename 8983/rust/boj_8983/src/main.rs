// https://www.acmicpc.net/problem/8983

use std::fmt::Debug;
use std::str::FromStr;
use std::str::SplitAsciiWhitespace;

trait IterParseHelper<'a>: Iterator<Item = &'a str> {
    fn parse_to<T>(&mut self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.next().unwrap().parse::<T>().unwrap()
    }
}

impl<'a> IterParseHelper<'a> for SplitAsciiWhitespace<'a> {}


fn solution(stdin: &str) {
    let mut tokens = stdin.split('\n');
    let mut next = || tokens.next().unwrap();

    let (_, n, l) = {
        let mut iter = next().split_ascii_whitespace();
        (
            iter.parse_to::<i32>(),
            iter.parse_to::<i32>(),
            iter.parse_to::<i32>(),
        )
    };

    let mut shoot_pos = next()
        .split_ascii_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    shoot_pos.sort_unstable(); // 이진 탐색을 위해 정렬

    let mut ret = 0_i32;
    (0..n)
        .map(|_| {
            let mut iter = next().split_ascii_whitespace();
            (iter.parse_to::<i32>(), iter.parse_to::<i32>())
        })
        .for_each(|(x, y)| {
            let (min, max) = (x + y - l, x - y + l);
            shoot_pos
                .binary_search_by(|&x| {
                    if x < min {
                        std::cmp::Ordering::Less
                    } else if x > max {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                })
                .is_ok()
                .then(|| ret += 1);
        });
    println!("{ret}");
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
            capture_output(|| solution("4 8 4\n6 1 4 9\n7 2\n3 3\n4 5\n5 1\n2 2\n1 4\n8 4\n9 4")),
            "6"
        );
    }

    #[test]
    fn test_solution2() {
        assert_eq!(
            capture_output(|| solution("1 5 3\n3\n2 2\n1 1\n5 1\n4 2\n3 3")),
            "5"
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
