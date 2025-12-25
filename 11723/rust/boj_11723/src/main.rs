// 집합
// https://www.acmicpc.net/problem/11723

use std::fmt::Write;

struct BitSet(u32);

impl BitSet {
    fn to_bit(value: u32) -> u32 {
        1 << (value - 1)
    }

    fn add(&mut self, value: u32) {
        self.0 |= Self::to_bit(value);
    }

    fn remove(&mut self, value: u32) {
        self.0 &= !(Self::to_bit(value));
    }

    fn check(&self, value: u32) -> bool {
        (self.0 & Self::to_bit(value)) != 0
    }

    fn toggle(&mut self, value: u32) {
        self.0 ^= Self::to_bit(value);
    }

    fn all(&mut self) {
        self.0 = Self::to_bit(21) - 1;
    }

    fn empty(&mut self) {
        self.0 = 0;
    }
}

fn main() {
    let mut bitset = BitSet(0);
    let mut output = String::with_capacity(3_000_000 * 2);

    let mut input = String::with_capacity(12);
    std::io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<u32>().unwrap();
    for _ in 0..n {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split_ascii_whitespace();
        match commands.next().unwrap() {
            "add" => {
                commands
                    .next()
                    .map(|v| v.parse::<u32>().unwrap())
                    .map(|v| bitset.add(v));
            }
            "remove" => {
                commands
                    .next()
                    .map(|v| v.parse::<u32>().unwrap())
                    .map(|v| bitset.remove(v));
            }
            "check" => {
                commands
                    .next()
                    .map(|v| v.parse::<u32>().unwrap())
                    .map(|v| writeln!(&mut output, "{}", bitset.check(v) as u32));
            }
            "toggle" => {
                commands
                    .next()
                    .map(|v| v.parse::<u32>().unwrap())
                    .map(|v| bitset.toggle(v));
            }
            "all" => {
                bitset.all();
            }
            "empty" => {
                bitset.empty();
            }
            _ => unreachable!(),
        }
    }

    print!("{}", output);
}
