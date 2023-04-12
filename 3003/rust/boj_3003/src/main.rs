use std::io::stdin;

fn main() {
    let chess_piece = vec![1, 1, 2, 2, 2, 8];

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    let pieces = buf
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i8>().unwrap())
        .collect::<Vec<_>>();

    for idx in 0..chess_piece.len() {
        print!(
            "{} ",
            chess_piece[idx] - pieces[idx],
        );
    }
}
