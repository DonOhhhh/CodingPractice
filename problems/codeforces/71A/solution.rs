use std::io::*;

fn main() -> Result<()> {
    let stdout = stdout().lock();
    let mut writer = BufWriter::new(stdout);

    let mut input = Vec::new();
    stdin().lock().read_to_end(&mut input)?;

    for word in input
        .split(|b| b.is_ascii_whitespace())
        .filter(|s| !s.is_empty())
        .skip(1)
    {
        let len = word.len();
        if len > 10 {
            writer.write_all(&[word[0]])?;
            write!(writer, "{}", len - 2)?;
            writer.write_all(&[word[len - 1], b'\n'])?;
        } else {
            writer.write_all(word)?;
            writer.write_all(&[b'\n'])?;
        }
    }

    writer.flush()?;
    Ok(())
}
