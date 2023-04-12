use std::{
    error::Error,
    io::{self, Write},
};
fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    let stdout = io::stdout().lock();
    let mut out = io::BufWriter::new(stdout);

    for _ in 0..buf.trim().parse::<usize>().unwrap() {
        buf.clear();
        io::stdin().read_line(&mut buf)?;

        write!(
            out,
            "{}\n",
            buf.trim()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .sum::<u32>()
        )?;
    }

    Ok(())
}
