use std::io::{self, Read, Write, BufWriter};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());

    iter.next();
    let mut res = Vec::new();
    for (i, &c) in iter.next().unwrap().as_bytes().to_vec().iter().rev().enumerate() {
        let pos = if i % 2 == 0 {
            res.len() - i / 2
        } else {
            i / 2
        };
        res.insert(pos, c);
    }
    writer.write_all(&mut res)?;
    Ok(())
}
