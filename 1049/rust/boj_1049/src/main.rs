// 기타줄
// https://www.acmicpc.net/problem/1049

fn solution(stdin: &str) {
    let mut lines = stdin.lines();

    let header: Vec<usize> = lines
        .next()
        .unwrap_or_default()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // n: 끊어진 줄 개수, m: 기타줄 브랜드 개수
    let (n, m) = (header[0], header[1]);

    // min_package: 최소 패키지 가격, min_single: 최소 낱개 가격
    let (min_package, min_single) = lines
        .take(m)
        .filter_map(|line| {
            let mut nums = line
                .split_ascii_whitespace()
                .map(|s| s.parse::<u32>().unwrap());
            Some((nums.next()?, nums.next()?))
        })
        .fold((u32::MAX, u32::MAX), |(acc_p, acc_s), (p, s)| {
            (acc_p.min(p), acc_s.min(s))
        });

    let package_count = (n / 6) as u32;
    let single_count = (n % 6) as u32;

    let total_cost = (min_package * package_count) + (min_single * single_count);
    let package_only_cost = min_package * (package_count + 1);
    let single_only_cost = min_single * (n as u32);

    println!("{}", total_cost.min(package_only_cost).min(single_only_cost));
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
        assert_eq!(capture_output(|| solution("4 2\n12 3\n15 4")), "12");
    }

    #[test]
    fn test_solution2() {
        assert_eq!(capture_output(|| solution("10 3\n20 8\n40 7\n60 4")), "36");
    }

    #[test]
    fn test_solution3() {
        assert_eq!(capture_output(|| solution("15 1\n100 40")), "300");
    }

    #[test]
    fn test_solution4() {
        assert_eq!(capture_output(|| solution("17 1\n12 3")), "36");
    }

    #[test]
    fn test_solution5() {
        assert_eq!(capture_output(|| solution("7 2\n10 3\n12 2")), "12");
    }

    #[test]
    fn test_solution6() {
        assert_eq!(
            capture_output(|| solution(
                "9 16\n21 25\n77 23\n23 88\n95 43\n96 19\n59 36\n80 13\n51 24\n15 8\n25 61\n21 22\n3 9\n68 68\n67 100\n83 98\n96 57"
            )),
            "6"
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
