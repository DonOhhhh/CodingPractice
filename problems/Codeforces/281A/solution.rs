use std::io::{self, Read, Write, BufWriter};

fn main() -> io::Result<()> {
    let mut input = Vec::new();
    io::stdin().lock().read_to_end(&mut input)?;

    input[0] &= !0x20;

    let mut writer = BufWriter::new(io::stdout().lock());
    writer.write_all(&mut input)?;
    Ok(())
}
