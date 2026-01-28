use std::io::{self, Read, Write, BufReader, BufWriter};

fn main() -> io::Result<()> {
    const CAPACITY: usize = 1 << 20;
    let mut input = String::with_capacity(CAPACITY);
    BufReader::new(io::stdin().lock()).read_to_string(&mut input)?;
    let mut writer = BufWriter::new(io::stdout().lock());

    for s in input.trim().split_ascii_whitespace().skip(2).step_by(2) {
        let mut moves = 0;
        let mut count = 0;
        for &b in s.as_bytes() {
            if b == b'(' {
                count += 1;
            } else {
                count -= 1;
                if count < 0 {
                    moves += 1;
                    count = 0;
                }
            }
        }
        writeln!(writer, "{}", moves)?;
    }
    Ok(())
}
