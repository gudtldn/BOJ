// https://www.acmicpc.net/problem/1620

use std::collections::HashMap;

fn solution(stdin: &str) {
    let mut tokens = stdin.split('\n');
    let mut next = || tokens.next().unwrap();

    let (n, m) = {
        let mut iter = next().split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };

    let mut pokemons = HashMap::new();
    let mut pokemons_rev = HashMap::new();

    for idx in 1..=n {
        let pokemon = next().trim().to_owned();
        pokemons.insert(pokemon.clone(), idx);
        pokemons_rev.insert(idx, pokemon);
    }

    for _ in 0..m {
        let query = next().trim();
        if let Ok(idx) = query.parse::<usize>() {
            println!("{}", pokemons_rev.get(&idx).unwrap());
        } else {
            println!("{}", pokemons.get(query).unwrap());
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
                &[
                    "26 5",
                    "Bulbasaur",
                    "Ivysaur",
                    "Venusaur",
                    "Charmander",
                    "Charmeleon",
                    "Charizard",
                    "Squirtle",
                    "Wartortle",
                    "Blastoise",
                    "Caterpie",
                    "Metapod",
                    "Butterfree",
                    "Weedle",
                    "Kakuna",
                    "Beedrill",
                    "Pidgey",
                    "Pidgeotto",
                    "Pidgeot",
                    "Rattata",
                    "Raticate",
                    "Spearow",
                    "Fearow",
                    "Ekans",
                    "Arbok",
                    "Pikachu",
                    "Raichu",
                    "25",
                    "Raichu",
                    "3",
                    "Pidgey",
                    "Kakuna",
                ]
                .join("\n")
            )),
            ["Pikachu", "26", "Venusaur", "16", "14",].join("\n")
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
