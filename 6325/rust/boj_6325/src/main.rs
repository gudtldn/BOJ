// Definite Values
// https://www.acmicpc.net/problem/6325

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

    let mut program_iter = 1_u32;
    loop {
        let n = parse_line!(usize);
        if n == 0 {
            return;
        }

        if program_iter != 1 {
            println!();
        }

        let mut variables = HashSet::new();
        variables.insert("a");

        for _ in 0..n {
            let (lhs, rhs) = {
                let mut splitted = next().split(" = ");
                (splitted.next().unwrap(), splitted.next().unwrap())
            };

            if variables.contains(rhs) {
                variables.insert(lhs);
            } else {
                variables.remove(lhs);
            }
        }

        println!("Program #{program_iter}");
        println!(
            "{}",
            if variables.is_empty() {
                "none".to_string()
            } else {
                let mut vars: Vec<_> = variables.iter().cloned().collect();
                vars.sort();
                vars.join(" ")
            }
        );

        program_iter += 1;
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
            capture_output(|| solution("4\nb = a\nc = d\nd = b\ne = f\n1\na = b\n0")),
            "Program #1\na b d\n\nProgram #2\nnone"
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
