// 스택 수열
// https://www.acmicpc.net/problem/1874

use std::fmt::Write;

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

    let mut next_value = 1;
    let mut stack = Vec::new();

    let mut buffer = String::new();
    for _ in 0..n {
        let check = parse_line!(usize);

        // next_value가 check와 같아질 때 까지 stack에 추가
        while next_value <= check {
            stack.push(next_value);
            next_value += 1;
            writeln!(&mut buffer, "+").unwrap();
        }

        // stack의 마지막 부분과 같다면 빼기
        if stack.pop_if(|&mut x| x == check).is_some() {
            writeln!(&mut buffer, "-").unwrap();
        } else {
            println!("NO");
            return;
        }
    }
    print!("{buffer}");
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
            capture_output(|| solution("8\n4\n3\n6\n8\n7\n5\n2\n1")),
            "+\n+\n+\n+\n-\n-\n+\n+\n-\n+\n+\n-\n-\n-\n-\n-"
        );
    }

    #[test]
    fn test_solution2() {
        assert_eq!(capture_output(|| solution("5\n1\n2\n5\n3\n4")), "NO");
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
