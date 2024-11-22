// https://www.acmicpc.net/problem/4949

fn solution(stdin: &str) {
    let mut tokens = stdin.split('\n');
    let mut next = || tokens.next().unwrap();

    'outer: loop {
        let line = next();
        if line == "." {
            break;
        }

        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        println!("no");
                        continue 'outer;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        println!("no");
                        continue 'outer;
                    }
                }
                _ => (),
            }
        }
        println!("{}", if stack.is_empty() { "yes" } else { "no" });
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
            capture_output(|| solution(
                "So when I die (the [first] I will see in (heaven) is a score list).
[ first in ] ( first out ).
Half Moon tonight (At least it is better than no Moon at all].
A rope may form )( a trail in a maze.
Help( I[m being held prisoner in a fortune cookie factory)].
([ (([( [ ] ) ( ) (( ))] )) ]).
 .
."
            )),
            "yes
yes
no
no
no
yes
yes"
            .to_string()
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
