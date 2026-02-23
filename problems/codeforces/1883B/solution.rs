use std::io::{self, BufReader, Read, Write};

macro_rules! scan {
    ($it:expr) => {
        $it.next().unwrap()
    };
    ($it:expr, $t:ty) => {
        $it.next().unwrap().parse::<$t>().unwrap()
    };
}

fn main() -> io::Result<()> {
    const CAPACITY: usize = 1 << 24;
    let mut input = String::with_capacity(CAPACITY);
    BufReader::with_capacity(CAPACITY, io::stdin().lock()).read_to_string(&mut input)?;
    let mut output = String::with_capacity(4096);
    let mut it = input.split_ascii_whitespace();
    let t = scan!(it, usize);
    for _ in 0..t {
        let (_, k) = (scan!(it, usize), scan!(it, usize));
        let s = scan!(it);
        let mut odd_count = [0; 26];
        for c in s.bytes() {
            odd_count[(c - b'a') as usize] += 1;
        }
        output.push_str(
            if k >= odd_count
                .iter()
                .filter(|&c| c % 2 == 1)
                .count()
                .saturating_sub(1)
            {
                "YES\n"
            } else {
                "NO\n"
            },
        )
    }
    io::stdout().lock().write_all(output.as_bytes())?;
    Ok(())
}
