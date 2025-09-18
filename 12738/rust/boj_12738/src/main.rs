// https://www.acmicpc.net/problem/12738

fn solution(stdin: &str) {
    let mut tokens = stdin.split('\n');
    let mut next = || tokens.next().unwrap();

    let _ = next(); // N은 무시

    // LIS를 담을 벡터
    let mut answer = vec![];

    // 수열을 순회하며 이분 탐색으로 위치를 찾고 갱신
    next()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .for_each(|x: i32| {
            // 이분 탐색으로 위치 찾기 (없으면 삽입 위치)
            let idx = answer.binary_search(&x).unwrap_or_else(|x| x);

            if idx == answer.len() {
                // 만약 x가 answer의 모든 원소보다 크다면 맨 뒤에 추가
                answer.push(x);
            } else {
                // 만약 x가 answer의 일부 원소보다 작거나 같다면 해당 위치 갱신
                answer[idx] = x;
            }
        });

    println!("{}", answer.len());
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
