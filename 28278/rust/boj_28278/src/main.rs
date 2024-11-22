// https://www.acmicpc.net/problem/28278

struct Stack {
    stack: Vec<i32>,
}

impl Stack {
    fn new() -> Self {
        Self { stack: Vec::with_capacity(1_000_000) }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap_or(-1)
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap_or(&-1)
    }
}

fn solution(stdin: &str) {
    let mut tokens = stdin.split('\n');
    let mut next = || tokens.next().unwrap();

    let mut stack = Stack::new();
    for _ in 0..next().trim().parse::<usize>().unwrap() {
        match next()
            .trim()
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .as_slice()
        {
            ["1", val] => stack.push(val.parse().unwrap()),
            ["2"] => println!("{}", stack.pop()),
            ["3"] => println!("{}", stack.len()),
            ["4"] => println!("{}", stack.is_empty() as i32),
            ["5"] => println!("{}", stack.top()),
            _ => (),
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
            capture_output(|| solution("9\n4\n1 3\n1 5\n3\n2\n5\n2\n2\n5")),
            "1\n2\n5\n3\n3\n-1\n-1".to_string()
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
