// Pyramids
// https://www.acmicpc.net/problem/5341

fn solution(stdin: &str) {
    let mut tokens = stdin.split('\n');
    let mut next = || tokens.next().unwrap();

    #[allow(unused_macros)]
    macro_rules! parse_line {
        ($($to_type:ty),*) => {
            {
                let mut iter = next().split_whitespace();
                ($(iter.next().unwrap().parse::<$to_type>().unwrap()),*)
            }
        };
    }

    loop {
        let n = parse_line!(i64);
        if n == 0 {
            break;
        }
        println!("{}", n * (n + 1) / 2);
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
        assert_eq!(capture_output(|| solution("4\n6\n0")), "10\n21");
    }

    #[test]
    fn test_solution2() {
        assert_eq!(capture_output(|| solution("5\n3\n0")), "15\n6");
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
