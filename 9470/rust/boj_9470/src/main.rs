// https://www.acmicpc.net/problem/9470

use std::collections::VecDeque;

#[derive(Clone, Copy, Default, Debug)]
struct StrahlerInfo {
    /// 이 노드가 의존하는 다른 노드들 중에서 가장 큰 strahler 순서
    max_order: u32,

    /// 이 노드가 의존하는 다른 노드의 수
    count: u32,
}

#[derive(Clone, Default, Debug)]
struct Node {
    /// 이 노드를 의존하는 다른 노드의 인덱스
    edges: Vec<usize>,

    /// 이 노드의 strahler 순서
    strahler: u32,
}

fn solution(stdin: &str) {
    let mut tokens = stdin.split('\n');
    let mut next = || tokens.next().unwrap();

    macro_rules! parse_line {
        ($($to_type:ty),*) => {
            {
                let mut iter = next().split_whitespace();
                ($(iter.next().unwrap().parse::<$to_type>().unwrap()),*)
            }
        };
    }

    let t = parse_line!(usize);
    for _ in 0..t {
        // k: 테스트 케이스 번호
        // m: 노드의 수
        // p: 간선의 수
        let (k, m, p) = parse_line!(u32, usize, usize);

        // 그래프 생성 및 진입 차수 계산 (각 노드가 몇 개의 다른 노드에 의존하는지 계산)
        let mut graph = vec![Node::default(); m]; // 인접 리스트, graph[a]는 a에서 출발하는 간선들이 도착하는 노드들의 리스트
        let mut graph_info = vec![StrahlerInfo::default(); m]; // 이 노드가 의존하는 다른 노드의 Strahler 정보

        let mut in_degrees = vec![0_u32; m]; // idx+1: 노드, value: 진입차수
        for _ in 0..p {
            let (a, b) = parse_line!(usize, usize);

            // B가 A에 의존, A -> B
            graph[a - 1].edges.push(b - 1); // A에서 B로 가는 간선 추가
            in_degrees[b - 1] += 1;         // B로 들어오는 간선이 하나 늘었으므로 진입 차수 1 증가
        }

        // 강의 근원(진입 차수가 0인 노드)을 큐에 추가
        let mut graph_idx_queue = VecDeque::new();
        for (n, &deg) in in_degrees.iter().enumerate() {
            if deg == 0 {
                // 큐에 추가된 노드는 위상 정렬의 시작점으로 사용
                graph_idx_queue.push_back(n);

                // node의 Strahler 정보를 기입
                graph[n].strahler = 1;       // 강의 근원은 strahler 순서가 1이기 때문에 1로 설정
                graph_info[n].max_order = 1; // 자기 자신의 순서는 1
                graph_info[n].count = 1;     // 순서가 1인 노드는 자기 자신 1개
            }
        }

        // 강의 근원부터 edges를 타고 가면서(위상 정렬을 하면서), strahler순서를 갱신
        while let Some(idx) = graph_idx_queue.pop_front() {
            // 만약 상류 노드가 2개 이상이면
            if graph_info[idx].count >= 2 {
                // 현재 노드는 상류 노드의 strahler순서 최대값의 + 1
                graph[idx].strahler = graph_info[idx].max_order + 1;
            }

            // edges는 graph[idx]에 의존하고 있는 다른 노드들 (하류 강)
            for edge_idx in graph[idx].edges.clone() {
                // 하류 노드의 진입 차수를 1 감소
                // 하류 노드의 입장에서는 처리해야 할 상류 노드가 하나 줄어든 것.
                in_degrees[edge_idx] -= 1;

                // 만약 현재까지의 strahler의 최대 순서가 현재 노드의 strahler순서와 같다면
                if graph_info[edge_idx].max_order == graph[idx].strahler {
                    // 그냥 상류 강 개수만 증가
                    graph_info[edge_idx].count += 1;
                }
                // 만약 현재까지의 strahler의 최대 순서가 현재 노드의 strahler순서보다 작다면
                else if graph_info[edge_idx].max_order < graph[idx].strahler {
                    // 하류 노드의 strahler순서를 상류 노드와 동일하게 설정
                    graph[edge_idx].strahler = graph[idx].strahler;

                    // StrahlerInfo를 갱신
                    graph_info[edge_idx].max_order = graph[idx].strahler; // 최대 순서 갱신
                    graph_info[edge_idx].count = 1;                       // 이렇게 되면 새로운 최대 순서를 가진 강은 현재 1개만 존재
                }

                // 진입 차수가 0이 되었다는건, 상류 노드의 계산이 끝났다는것을 의미하므로 큐에 추가
                if in_degrees[edge_idx] == 0 {
                    graph_idx_queue.push_back(edge_idx);
                }
            }
        }

        // 마지막 노드의 strahler 순서를 출력
        println!("{} {}", k, graph[m - 1].strahler);
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
            capture_output(|| solution("1\n1 7 8\n1 3\n2 3\n6 4\n3 4\n3 5\n6 7\n5 7\n4 7")),
            "1 3"
        );
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
