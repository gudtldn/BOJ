// https://www.acmicpc.net/problem/14425

use std::collections::HashSet;

fn solution(stdin: &str) {
    let mut tokens = stdin.split('\n');
    let mut next = || tokens.next().unwrap();

    let (n, m) = {
        let mut iter = next().split_whitespace();
        let n: i32 = iter.next().unwrap().parse().unwrap();
        let m: i32 = iter.next().unwrap().parse().unwrap();
        (n, m)
    };

    // 집합 s
    let mut set_s = HashSet::new();
    for _ in 0..n {
        set_s.insert(next().trim());
    }

    let mut count = 0;
    for _ in 0..m {
        let s = next().trim();
        if set_s.contains(s) {
            count += 1;
        }
    }

    println!("{count}");
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
            capture_output(|| solution(
                &[
                    "5 11",
                    "baekjoononlinejudge",
                    "startlink",
                    "codeplus",
                    "sundaycoding",
                    "codingsh",
                    "baekjoon",
                    "codeplus",
                    "codeminus",
                    "startlink",
                    "starlink",
                    "sundaycoding",
                    "codingsh",
                    "codinghs",
                    "sondaycoding",
                    "startrink",
                    "icerink",
                ]
                .join("\n")
            )),
            "4"
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
