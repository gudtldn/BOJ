use std::{error::Error, io, fmt::Write};

fn seg_init(arr: &Vec<u32>, seg_tree: &mut Vec<u32>, node: usize, start: usize, end: usize) {
    if start == end {
        seg_tree[node] = arr[start];
    } else {
        let mid = (start + end) / 2;
        seg_init(arr, seg_tree, node * 2, start, mid);
        seg_init(arr, seg_tree, node * 2 + 1, mid + 1, end);
        seg_tree[node] = seg_tree[node * 2].min(seg_tree[node * 2 + 1]);
    }
}

fn seg_query(
    seg_tree: &Vec<u32>,
    node: usize,
    start: usize,
    end: usize,
    left: usize,
    right: usize,
) -> u32 {
    if right < start || end < left {
        return std::u32::MAX;
    }

    if left <= start && end <= right {
        return seg_tree[node];
    }

    let mid = (start + end) / 2;
    seg_query(seg_tree, node * 2, start, mid, left, right).min(seg_query(
        seg_tree,
        node * 2 + 1,
        mid + 1,
        end,
        left,
        right,
    ))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let (n, m) = {
        let mut iter = input.trim().split_whitespace();
        (
            iter.next().unwrap().parse::<usize>()?,
            iter.next().unwrap().parse::<usize>()?,
        )
    };

    let mut arr = vec![0; n];
    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        arr[i] = input.trim().parse::<u32>()?;
    }

    let mut seg_tree = vec![u32::MAX; n * 4];
    seg_init(&arr, &mut seg_tree, 1, 0, n - 1);

    let mut print_buf = String::new();
    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let (a, b) = {
            let mut iter = input.trim().split_whitespace();
            (
                iter.next().unwrap().parse::<usize>()?,
                iter.next().unwrap().parse::<usize>()?,
            )
        };
        writeln!(&mut print_buf, "{}", seg_query(&seg_tree, 1, 0, n - 1, a - 1, b - 1))?;
    }

    print!("{}", print_buf);
    Ok(())
}
