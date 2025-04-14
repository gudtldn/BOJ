
fn solution(stdin: &str) {
    let mut tokens = stdin.split('\n');
    let mut next = || tokens.next().unwrap();

    let n: usize = next().trim().parse().unwrap();
    let a: Vec<i32> = next()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut dp = vec![1; n + 1];
    for i in 1..n {
        for j in 0..i {
            // 비교하다가 a[i]보다 작은 수를 만나면 기록
            if a[i] > a[j] {
                dp[i] = dbg!(dp[i].max(dp[j] + 1));
                dbg!(&dp);
            }
        }
    }
    println!("{}", dp.iter().max().unwrap()); // 최대값을 출력
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
        assert_eq!(capture_output(|| solution("6\n10 20 10 30 20 50")), "4");
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
