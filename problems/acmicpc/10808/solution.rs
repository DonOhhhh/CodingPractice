use std::io::{self, BufWriter, Read, Write};

fn main() -> io::Result<()> {
    let mut input = Vec::with_capacity(101);
    io::stdin().lock().read_to_end(&mut input)?;

    let mut count = [0u8; 26];
    for &e in &input {
        if b'a' <= e && e <= b'z' {
            count[(e - b'a') as usize] += 1;
        }
    }

    let stdout = io::stdout().lock();
    let mut writer = BufWriter::new(stdout);

    for &c in &count {
        write!(writer, "{} ", c)?;
    }
    writer.flush()?;
    Ok(())
}
