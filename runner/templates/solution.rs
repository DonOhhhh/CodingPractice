use std::io::{self, Read, Write, BufWriter};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();

    let mut writer = BufWriter::new(io::stdout().lock());
    writer.write_all(&mut input)?;
    Ok(())
}
