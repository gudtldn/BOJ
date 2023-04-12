use std::{io::stdin, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;

    for _ in 0..buf.trim().parse::<usize>().unwrap() {
        let mut buf = String::new();
        stdin().read_line(&mut buf)?;

        let ox_chars = buf.trim().chars();
        
        let mut total = 0u32;
        let mut x = 0u32;

        for ox in ox_chars {
            x += 1;

            match ox {
                'O' => {
                    total += x;
                },
                'X' => {
                    x = 0;
                },
                _ => ()
            }
        }

        println!("{}", total);
    }

    Ok(())
}
