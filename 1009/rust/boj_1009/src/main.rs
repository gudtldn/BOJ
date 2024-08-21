// https://www.acmicpc.net/problem/1009

fn fast_pow(mut a: i64, mut x: i64, n: i64) -> i64 {
    let mut y = 1;
    while x > 0 {
        if x & 1 == 1 {
            y = (a * y) % n;
        }
        a = (a * a) % n;
        x = x >> 1;
    }
    y
}

fn solution(stdin: &str) -> String {
    let mut tokens = stdin.split('\n');
    let mut next = || tokens.next().unwrap();
    let mut buffer = Vec::<String>::new();

    for _ in 0..next().parse::<i64>().unwrap() {
        let mut ab = next().split_whitespace().map(|x| x.parse::<i64>().unwrap());
        let a = ab.next().unwrap();
        let b = ab.next().unwrap();
        let result = fast_pow(a, b, 10) % 10;
        buffer.push((if result == 0 { 10 } else { result }).to_string());
    }

    buffer.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        assert_eq!(solution("5\n1 6\n3 7\n6 2\n7 100\n9 635"), "1\n7\n6\n1\n9");
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solution("1\n1 1"), "1");
    }

    #[test]
    fn test_solution3() {
        assert_eq!(solution("1\n2 2"), "4");
    }

    #[test]
    fn test_solution4() {
        assert_eq!(solution("1\n99 999999"), "9");
    }

    #[test]
    fn test_solution5() {
        assert_eq!(solution("1\n10 10"), "10");
    }
}

fn main() {
    use std::io::*;
    println!("{}", solution(&read_to_string(stdin()).unwrap()));
    STDOUT.with(|refcell| std::io::Write::flush(&mut *refcell.borrow_mut()).unwrap());
}

thread_local! {
    static STDOUT: std::cell::RefCell<std::io::BufWriter<std::io::StdoutLock<'static>>> = std::cell::RefCell::new(std::io::BufWriter::with_capacity(1 << 17, std::io::stdout().lock()));
}
