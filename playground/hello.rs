use std::io::{self, *};

fn main() -> Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());

    let mut input = Vec::new();
    reader.read_to_end(&mut input)?;

    let result = input.split(|&x| x == b' ').count();

    writeln!(writer, "{}", result)?;
    Ok(())
}
