use std::io::*;
fn main() -> Result<()> {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let mut result = 0;
    if let Some(data) = buf.split(|&b| b == 10).nth(1) {
        result = data.iter().map(|&x| (x - b'0') as i32).sum::<i32>();
    }

    writeln!(writer, "{:?}", result)?;
    Ok(())
}
