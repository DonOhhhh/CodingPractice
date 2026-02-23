use std::io::{self, BufWriter, Read, Write};

fn main() -> io::Result<()> {
    let mut input = Vec::with_capacity(1 << 7);
    io::stdin().lock().read_to_end(&mut input)?;
    input = input.into_iter().filter(|&x| !x.is_ascii_whitespace()).collect();

    let lowercase_count = input.iter().filter(|&&c| c >= b'a' && c <= b'z').count();
    let uppercase_count = input.len() - lowercase_count;
    let mut output = Vec::with_capacity(1 << 7);
    for &c in &input {
        if lowercase_count >= uppercase_count {
            output.push(c | 0x20);
        } else {
            output.push(c & !0x20);
        }
    }

    let mut writer = BufWriter::new(io::stdout().lock());
    writer.write_all(&mut output)?;
    Ok(())
}
