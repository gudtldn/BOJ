// https://www.acmicpc.net/problem/2609

fn gdc(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: i32, b: i32) -> i32 {
    a * b / gdc(a, b)
}

fn solution(stdin: &str) -> String {
    let mut tokens = stdin.split('\n');
    let mut next = || tokens.next().unwrap();
    let mut buffer = Vec::<String>::new();

    let (a, b) = {
        let mut it = next().split_whitespace().map(|x| x.parse().unwrap());
        (it.next().unwrap(), it.next().unwrap())
    };

    buffer.push(gdc(a, b).to_string());
    buffer.push(lcm(a, b).to_string());

    buffer.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        assert_eq!(solution("24 18"), "6\n72".to_string());
    }
    /* add more tests */
}

fn main() {
    use std::io::*;
    println!("{}", solution(&read_to_string(stdin()).unwrap()));
    STDOUT.with(|refcell| std::io::Write::flush(&mut *refcell.borrow_mut()).unwrap());
}

thread_local! {
    static STDOUT: std::cell::RefCell<std::io::BufWriter<std::io::StdoutLock<'static>>> = std::cell::RefCell::new(std::io::BufWriter::with_capacity(1 << 17, std::io::stdout().lock()));
}
