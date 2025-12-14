// 계단 오르기
// https://www.acmicpc.net/problem/2579

fn solution(stdin: &str) {
    let mut tokens = stdin.split_ascii_whitespace();

    // 계단 개수와 점수 입력
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let stairs: Vec<u32> = tokens
        .map(|token| token.parse().unwrap())
        .collect();

    if n == 1 {
        println!("{}", stairs[0]);
        return;
    } else if n == 2 {
        println!("{}", stairs[0] + stairs[1]);
        return;
    }

    // idx 번째 계단에 도달했을 때의 최대 점수
    let mut dp = vec![0; n];
    dp[0] = stairs[0];                                          // 1번째 계단은 무조건 밟음
    dp[1] = stairs[0] + stairs[1];                              // 2번째 계단도 무조건 밟음
    dp[2] = (stairs[0] + stairs[2]).max(stairs[1] + stairs[2]); // 3번째 계단은 1번째 or 2번째 계단에서 올 수 있음

    // 점화식에 따라 dp 배열 채우기
    for i in 3..n {
        // dp[i] = max(
        //     dp[i-2] + stairs[i],              // 2칸 점프
        //     dp[i-3] + stairs[i-1] + stairs[i] // 직전에 2칸 점프 후 1칸 점프
        // )
        dp[i] = (dp[i - 2] + stairs[i]).max(dp[i - 3] + stairs[i - 1] + stairs[i]);
    }

    // 마지막 계단에 도달했을 때의 최대 점수 출력
    println!("{}", dp[n - 1]);
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
            capture_output(|| solution("6\n10\n20\n15\n25\n10\n20")),
            "75"
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
