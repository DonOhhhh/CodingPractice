use std::io::{self, Read, Write, BufWriter};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let output : String = {
        let mut nums: Vec<&str> = input.trim().split('+').collect();
        nums.sort();
        nums.join("+")
    };

    let mut writer = BufWriter::new(io::stdout().lock());
    write!(writer, "{}", output)?;
    Ok(())
}
