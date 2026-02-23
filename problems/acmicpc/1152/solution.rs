use std::io::{self, *};

fn main() -> Result<()> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());

    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let result = input.split_whitespace().count();

    writeln!(writer, "{}", result)?;
    Ok(())
}
