// 큐빙
// https://www.acmicpc.net/problem/5373

/*
[g g g]   [w w w]
[g g g]   [w w w]
[g g g]   [w w w]

          [r r r]
          [r r r]
          [r r r]

          [y y y]
          [y y y]
          [y y y]

          [o o o]   [b b b]
          [o o o]   [b b b]
          [o o o]   [b b b]
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Faces {
    Left = 0,
    Up = 1,
    Front = 2,
    Down = 3,
    Back = 4,
    Right = 5,
}

impl Faces {
    fn idx(self) -> usize {
        self as usize
    }
}

impl From<char> for Faces {
    fn from(c: char) -> Self {
        match c {
            'L' => Faces::Left,
            'U' => Faces::Up,
            'F' => Faces::Front,
            'D' => Faces::Down,
            'B' => Faces::Back,
            'R' => Faces::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Color {
    Green,
    White,
    Red,
    Yellow,
    Orange,
    Blue,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Color::Green => 'g',
            Color::White => 'w',
            Color::Red => 'r',
            Color::Yellow => 'y',
            Color::Orange => 'o',
            Color::Blue => 'b',
        };
        write!(f, "{}", c)
    }
}

/// 0: 왼쪽, 1: 위쪽, 2: 앞쪽, 3: 아래쪽, 4: 뒤쪽, 5: 오른쪽
struct Cube {
    faces: [[[Color; 3]; 3]; 6],
}

impl Cube {
    fn new() -> Self {
        Self {
            faces: [
                [[Color::Green; 3]; 3],
                [[Color::White; 3]; 3],
                [[Color::Red; 3]; 3],
                [[Color::Yellow; 3]; 3],
                [[Color::Orange; 3]; 3],
                [[Color::Blue; 3]; 3],
            ],
        }
    }

    fn rotate_face_cw(&mut self, face: Faces) {
        let idx = face.idx();
        self.faces[idx] = [
            [
                self.faces[idx][2][0],
                self.faces[idx][1][0],
                self.faces[idx][0][0],
            ],
            [
                self.faces[idx][2][1],
                self.faces[idx][1][1],
                self.faces[idx][0][1],
            ],
            [
                self.faces[idx][2][2],
                self.faces[idx][1][2],
                self.faces[idx][0][2],
            ],
        ];
    }

    fn get_face(&self, face: Faces) -> [[Color; 3]; 3] {
        self.faces[face.idx()]
    }

    fn get_row(&self, face: Faces, row: usize) -> [Color; 3] {
        self.faces[face.idx()][row]
    }

    fn get_col(&self, face: Faces, col: usize) -> [Color; 3] {
        [
            self.faces[face.idx()][0][col],
            self.faces[face.idx()][1][col],
            self.faces[face.idx()][2][col],
        ]
    }

    fn up_cw(&mut self) {
        self.rotate_face_cw(Faces::Up);
        let mut temp = self.faces;

        for (n, &face) in self.get_row(Faces::Front, 0).iter().enumerate() {
            temp[Faces::Left.idx()][n][2] = face;
        }

        for (n, &face) in self.get_col(Faces::Left, 2).iter().rev().enumerate() {
            temp[Faces::Back.idx()][2][n] = face;
        }

        for (n, &face) in self.get_row(Faces::Back, 2).iter().enumerate() {
            temp[Faces::Right.idx()][2][n] = face;
        }

        for (n, &face) in self.get_row(Faces::Right, 2).iter().rev().enumerate() {
            temp[Faces::Front.idx()][0][n] = face;
        }

        self.faces = temp;
    }

    fn up_ccw(&mut self) {
        for _ in 0..3 {
            self.up_cw();
        }
    }

    fn down_cw(&mut self) {
        self.rotate_face_cw(Faces::Down);
        let mut temp = self.faces;

        for (n, &face) in self.get_row(Faces::Right, 0).iter().enumerate() {
            temp[Faces::Back.idx()][0][n] = face;
        }

        for (n, &face) in self.get_row(Faces::Back, 0).iter().rev().enumerate() {
            temp[Faces::Left.idx()][n][0] = face;
        }

        for (n, &face) in self.get_col(Faces::Left, 0).iter().enumerate() {
            temp[Faces::Front.idx()][2][n] = face;
        }

        for (n, &face) in self.get_row(Faces::Front, 2).iter().rev().enumerate() {
            temp[Faces::Right.idx()][0][n] = face;
        }

        self.faces = temp;
    }

    fn down_ccw(&mut self) {
        for _ in 0..3 {
            self.down_cw();
        }
    }

    fn front_cw(&mut self) {
        self.rotate_face_cw(Faces::Front);
        let mut temp = self.faces;

        for (n, &face) in self.get_row(Faces::Down, 0).iter().rev().enumerate() {
            temp[Faces::Left.idx()][2][n] = face;
        }

        for (n, &face) in self.get_row(Faces::Left, 2).iter().enumerate() {
            temp[Faces::Up.idx()][2][n] = face;
        }

        for (n, &face) in self.get_row(Faces::Up, 2).iter().rev().enumerate() {
            temp[Faces::Right.idx()][n][2] = face;
        }

        for (n, &face) in self.get_col(Faces::Right, 2).iter().enumerate() {
            temp[Faces::Down.idx()][0][n] = face;
        }

        self.faces = temp;
    }

    fn front_ccw(&mut self) {
        for _ in 0..3 {
            self.front_cw();
        }
    }

    fn back_cw(&mut self) {
        self.rotate_face_cw(Faces::Back);
        let mut temp = self.faces;

        for (n, &face) in self.get_row(Faces::Up, 0).iter().enumerate() {
            temp[Faces::Left.idx()][0][n] = face;
        }

        for (n, &face) in self.get_row(Faces::Left, 0).iter().rev().enumerate() {
            temp[Faces::Down.idx()][2][n] = face;
        }

        for (n, &face) in self.get_row(Faces::Down, 2).iter().enumerate() {
            temp[Faces::Right.idx()][n][0] = face;
        }

        for (n, &face) in self.get_col(Faces::Right, 0).iter().rev().enumerate() {
            temp[Faces::Up.idx()][0][n] = face;
        }

        self.faces = temp;
    }

    fn back_ccw(&mut self) {
        for _ in 0..3 {
            self.back_cw();
        }
    }

    fn left_cw(&mut self) {
        self.rotate_face_cw(Faces::Left);
        let mut temp = self.faces;

        for (n, &face) in self.get_col(Faces::Front, 0).iter().enumerate() {
            temp[Faces::Down.idx()][n][0] = face;
        }

        for (n, &face) in self.get_col(Faces::Down, 0).iter().enumerate() {
            temp[Faces::Back.idx()][n][0] = face;
        }

        for (n, &face) in self.get_col(Faces::Back, 0).iter().enumerate() {
            temp[Faces::Up.idx()][n][0] = face;
        }

        for (n, &face) in self.get_col(Faces::Up, 0).iter().enumerate() {
            temp[Faces::Front.idx()][n][0] = face;
        }

        self.faces = temp;
    }

    fn left_ccw(&mut self) {
        for _ in 0..3 {
            self.left_cw();
        }
    }

    fn right_cw(&mut self) {
        self.rotate_face_cw(Faces::Right);
        let mut temp = self.faces;

        for (n, &face) in self.get_col(Faces::Up, 2).iter().enumerate() {
            temp[Faces::Back.idx()][n][2] = face;
        }

        for (n, &face) in self.get_col(Faces::Back, 2).iter().enumerate() {
            temp[Faces::Down.idx()][n][2] = face;
        }

        for (n, &face) in self.get_col(Faces::Down, 2).iter().enumerate() {
            temp[Faces::Front.idx()][n][2] = face;
        }

        for (n, &face) in self.get_col(Faces::Front, 2).iter().enumerate() {
            temp[Faces::Up.idx()][n][2] = face;
        }

        self.faces = temp;
    }

    fn right_ccw(&mut self) {
        for _ in 0..3 {
            self.right_cw();
        }
    }
}

fn solution(stdin: &str) {
    let mut tokens = stdin.split('\n');
    let mut next = || tokens.next().unwrap();

    #[allow(unused_macros)]
    macro_rules! parse_line {
        ($($to_type:ty),*) => {
            {
                let mut iter = next().split_whitespace();
                ($(iter.next().unwrap().parse::<$to_type>().unwrap()),*)
            }
        };
    }

    for _ in 0..parse_line!(usize) {
        next(); // n은 무시

        let mut cube = Cube::new();
        for cmd in next().split_whitespace() {
            let chars = cmd.chars().collect::<Vec<_>>();
            let face: Faces = chars[0].into();
            let is_clockwise = chars[1] == '+';

            match (face, is_clockwise) {
                (Faces::Up, true) => cube.up_cw(),
                (Faces::Up, false) => cube.up_ccw(),
                (Faces::Down, true) => cube.down_cw(),
                (Faces::Down, false) => cube.down_ccw(),
                (Faces::Front, true) => cube.front_cw(),
                (Faces::Front, false) => cube.front_ccw(),
                (Faces::Back, true) => cube.back_cw(),
                (Faces::Back, false) => cube.back_ccw(),
                (Faces::Left, true) => cube.left_cw(),
                (Faces::Left, false) => cube.left_ccw(),
                (Faces::Right, true) => cube.right_cw(),
                (Faces::Right, false) => cube.right_ccw(),
            }
        }

        let up_face = cube.get_face(Faces::Up);
        for row in &up_face {
            for &color in row {
                print!("{}", color);
            }
            println!();
        }
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
                "4\n1\nL-\n2\nF+ B+\n4\nU- D- L+ R+\n10\nL- U- L+ U- L- U- U- L+ U+ U+"
            )),
            "rww\nrww\nrww\nbbb\nwww\nggg\ngwg\nowr\nbwb\ngwo\nwww\nrww"
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
