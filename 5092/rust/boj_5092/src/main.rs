// Air Old Zeeland
// https://www.acmicpc.net/problem/5092

use std::collections::HashMap;

#[derive(Debug)]
struct ProductInfo {
    loyalty: u32,
    wait_days: u32,
}

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

    loop {
        // 0 < N <= 50
        let n = parse_line!(usize);
        if n == 0 {
            return;
        }

        let mut products = HashMap::with_capacity(n);

        for _ in 0..n {
            let (name, loyalty, wait_days) = parse_line!(String, u32, u32);
            products.insert(name, ProductInfo { loyalty, wait_days });
        }

        let c = parse_line!(usize);
        let mut sum_discontented = 0;

        for _ in 0..c {
            let mut sum_loyalty = 0;
            let mut is_disconnected = false;
            let (user_id, count, max_wait_days) = parse_line!(u32, u32, u32);

            for _ in 0..count {
                let name = next();
                let product = &products[name];

                if product.wait_days > max_wait_days {
                    is_disconnected = true;
                    continue;
                }

                sum_loyalty += product.loyalty;
            }

            println!(
                "{} {}{}",
                user_id, sum_loyalty,
                if is_disconnected {
                    sum_discontented += 1;
                    " *"
                } else {
                    ""
                }
            );
        }

        println!("Number of discontented customers is: {sum_discontented}");
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
                "3\niPodNano 255 0\nDucksFeetPerfume 120 15\nSilverCharmPendant 180 3\n3\n1001 2 3\niPodNano\nSilverCharmPendant\n1860 1 5\nDucksFeetPerfume\n1025 2 6\niPodNano\nDucksFeetPerfume\n0"
            )),
            "1001 435\n1860 0 *\n1025 255 *\nNumber of discontented customers is: 2"
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
