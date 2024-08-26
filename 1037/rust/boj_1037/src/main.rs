// https://www.acmicpc.net/problem/1037

fn solution(stdin: &str) {
    let mut tokens = stdin.split('\n');
    let mut next = || tokens.next().unwrap();

    let n = next().parse::<usize>().unwrap();
    let mut vec = Vec::with_capacity(n);

    for i in next().split_whitespace() {
        vec.push(i.parse::<usize>().unwrap());
    }

    vec.sort_unstable();

    println!("{}", vec[0] * vec[n - 1]);
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
        assert_eq!(capture_output(|| solution("2\n4 2")), "8");
    }

    #[test]
    fn test_solution2() {
        assert_eq!(capture_output(|| solution("1\n2")), "4");
    }

    #[test]
    fn test_solution3() {
        assert_eq!(capture_output(|| solution("6\n3 4 2 12 6 8")), "24");
    }

    #[test]
    fn test_solution4() {
        assert_eq!(capture_output(|| solution("14\n14 26456 2 28 13228 3307 7 23149 8 6614 46298 56 4 92596")), "185192");
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
