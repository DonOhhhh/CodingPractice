use std::io::{self, BufWriter, Read, Write};

macro_rules! scan {
    ($it: expr, $t: ty) => {
        $it.next().unwrap().parse::<$t>().unwrap()
    };
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut stack: Vec<u8> = Vec::with_capacity(2 * 100_000);
    for _ in 0..scan!(iter, usize) {
        stack.clear();
        let s = iter.next().unwrap().as_bytes();
        for &b in s {
            if stack.is_empty() {
                stack.push(b);
            } else {
                if b == b'B' {
                    stack.pop();
                } else {
                    stack.push(b);
                }
            }
        }
        writeln!(writer, "{}", stack.len())?;
    }
    Ok(())
}
