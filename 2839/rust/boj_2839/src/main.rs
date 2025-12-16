// 설탕 배달
// https://www.acmicpc.net/problem/2839


#[allow(unused)]
fn greedy_solution(stdin: &str) {
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

    // 3 <= n <= 5000
    let n = parse_line!(u32);

    let mut count = n / 5;
    let mut remainder = n % 5;

    loop {
        if remainder % 3 == 0 {
            count += remainder / 3;
            break;
        } else {
            if count == 0 {
                println!("-1");
                return;
            }
            count -= 1;
            remainder += 5;
        }
    }

    println!("{}", count);
}

fn dp_solution(stdin: &str) {
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

    // 3 <= n <= 5000
    let n = parse_line!(usize);

    const MAX_WEIGHT: u32 = 5001;
    let mut dp = vec![MAX_WEIGHT; n + 1];

    // i: 현재 무게
    // dp[i]: 현재 무게에서 가질 수 있는 최적의 설탕 봉지 개수 합

    // i는 2가지 경우에서 올 수 있음
    // i-3: 3kg 봉지 추가
    // i-5: 5kg 봉지 추가

    // 기본값 설정
    if n >= 3 { dp[3] = 1; }
    if n >= 5 { dp[5] = 1; }

    for i in 6..=n {
        // 3kg 전에서 오는 경우와 5kg 전에서 오는 경우 중 더 작은 값 선택
        dp[i] = std::cmp::min(dp[i - 3], dp[i - 5]) + 1; // 봉지 하나 추가
    }

    if dp[n] >= MAX_WEIGHT {
        println!("-1");
    } else {
        println!("{}", dp[n]);
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
        assert_eq!(capture_output(|| greedy_solution("18")), "4");
        assert_eq!(capture_output(|| dp_solution("18")), "4");
    }

    #[test]
    fn test_solution2() {
        assert_eq!(capture_output(|| greedy_solution("4")), "-1");
        assert_eq!(capture_output(|| dp_solution("4")), "-1");
    }

    #[test]
    fn test_solution3() {
        assert_eq!(capture_output(|| greedy_solution("6")), "2");
        assert_eq!(capture_output(|| dp_solution("6")), "2");
    }

    #[test]
    fn test_solution4() {
        assert_eq!(capture_output(|| greedy_solution("9")), "3");
        assert_eq!(capture_output(|| dp_solution("9")), "3");
    }

    #[test]
    fn test_solution5() {
        assert_eq!(capture_output(|| greedy_solution("11")), "3");
        assert_eq!(capture_output(|| dp_solution("11")), "3");
    }
}

fn main() {
    use std::io::*;
    dp_solution(&read_to_string(stdin()).unwrap());
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
