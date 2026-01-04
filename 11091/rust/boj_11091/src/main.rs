// 알파벳 전부 쓰기
// https://www.acmicpc.net/problem/11091

use std::collections::HashSet;

fn solution(stdin: &str) {
    let mut lines = stdin.lines();
    let mut next = || lines.next().unwrap();

    #[allow(unused_macros)]
    macro_rules! parse_line {
        ($($to_type:ty),*) => {
            {
                let mut iter = next().split_whitespace();
                ($(iter.next().unwrap().parse::<$to_type>().unwrap()),*)
            }
        };
    }

    let n = parse_line!(usize);
    for _ in 0..n {
        let mut alphabet_set: HashSet<char> = ('a'..='z').collect();
        next().to_lowercase().chars().for_each(|c| {
            alphabet_set.remove(&c);
        });

        if alphabet_set.is_empty() {
            println!("pangram");
        } else {
            let mut missing_chars: Vec<char> = alphabet_set.into_iter().collect();
            missing_chars.sort_unstable();

            println!("missing {}", missing_chars.into_iter().collect::<String>());
        }
    }
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
            capture_output(|| solution("3\nThe quick brown fox jumps over the lazy dog.\nZYXW, vu TSR Ponm lkj ihgfd CBA.\n.,?!'\" 92384 abcde FGHIJ")),
            "pangram\nmissing eq\nmissing klmnopqrstuvwxyz"
        );
    }

}

fn main() {
    use std::io::*;
    solution(&read_to_string(stdin()).unwrap());
    STDOUT.with(|refcell| std::io::Write::flush(&mut *refcell.borrow_mut()).unwrap());
}

thread_local! {
    #[cfg(test)]
    static STDOUT: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(Vec::new());

    #[cfg(not(test))]
    static STDOUT: std::cell::RefCell<std::io::BufWriter<std::io::StdoutLock<'static>>> =
        std::cell::RefCell::new(std::io::BufWriter::with_capacity(1 << 17, std::io::stdout().lock()));
}

#[macro_export]
macro_rules! println {
    ($($t:tt)*) => {
        STDOUT.with(|refcell| {
            use std::io::*;
            writeln!(refcell.borrow_mut(), $($t)*).unwrap();
        })
    };
}

#[macro_export]
macro_rules! print {
    ($($t:tt)*) => {
        STDOUT.with(|refcell| {
            use std::io::*;
            write!(refcell.borrow_mut(), $($t)*).unwrap();
        })
    };
}
