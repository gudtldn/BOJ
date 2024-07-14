// https://www.acmicpc.net/problem/31001

use std::collections::HashMap;
use std::io::{BufRead, BufReader};

type StockInfo = HashMap<String, i64>;
type GroupInfo = HashMap<i64, Vec<String>>;

fn parse_value(value: &str) -> (i64, String, i64) {
    let mut parts = value.split_ascii_whitespace();
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().to_string(),
        parts.next().unwrap().parse().unwrap(),
    )
}

fn main() {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());

    // 주식 시장에 상장한 회사의 개수 N
    // 하이비가 보유하고 있는 현금 money
    // 메뉴 입력의 개수 Q
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    let (n, mut money, q) = {
        let v: Vec<i64> = buf.split_whitespace().flat_map(str::parse).collect();
        (v[0], v[1], v[2])
    };

    // 주식 정보를 담을 해시맵
    let mut stock_info: StockInfo = HashMap::new();
    let mut group_info: GroupInfo = HashMap::new();
    for _ in 0..n {
        buf.clear();
        reader.read_line(&mut buf).unwrap();
        let (group, k, v) = parse_value(&buf);
        stock_info.insert(k.clone(), v);
        group_info.entry(group).or_default().push(k);
    }

    // 하이비가 보유한 주식 정보를 담을 해시맵 및 로직
    let mut my_stock_info: StockInfo = HashMap::new();
    for _ in 0..q {
        buf.clear();
        reader.read_line(&mut buf).unwrap();

        let trimmed = buf.trim();
        match trimmed {
            // 하이비가 보유하고 있는 돈 출력
            "6" => println!("{money}"),

            // 하이비가 보유하고 있는 모든 자산의 가치 출력
            "7" => println!(
                "{}",
                money
                    + my_stock_info
                        .iter()
                        .map(|(k, v)| stock_info[k] * v)
                        .sum::<i64>()
            ),

            // 그 외의 메뉴
            _ => {
                let (menu, key, value) = parse_value(trimmed);
                match menu {
                    // key회사의 주식을 value개 구매
                    1 => {
                        let price = stock_info[&key] * value;
                        if money >= price {
                            money -= price;
                            *my_stock_info.entry(key).or_default() += value;
                        }
                    }

                    // key회사의 주식을 value개 판매
                    2 => {
                        if let Some(user_stock_count) = my_stock_info.get_mut(&key) {
                            let sell_amount = value.min(*user_stock_count);
                            *user_stock_count -= sell_amount;
                            money += stock_info[&key] * sell_amount;
                        }
                    }

                    // key회사의 주가를 value원 올리기
                    3 => *stock_info.get_mut(&key).unwrap() += value,

                    // key그룹에 속한 회사들의 주가를 value원 올리기
                    4 => {
                        let group_key: i64 = key.parse().unwrap();
                        for k in &group_info[&group_key] {
                            *stock_info.get_mut(k).unwrap() += value;
                        }
                    }

                    // key그룹에 속한 회사들의 주가를 value% 올리기
                    5 => {
                        let group_key: i64 = key.parse().unwrap();
                        for k in &group_info[&group_key] {
                            let stock_value = stock_info.get_mut(k).unwrap();
                            *stock_value = (*stock_value * (100 + value) / 100) / 10 * 10;
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}
