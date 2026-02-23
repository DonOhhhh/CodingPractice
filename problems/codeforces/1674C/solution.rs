use std::io::{self, BufWriter, Read, Write};

macro_rules! scan {
    ($it: expr, $t: ty) => {
        $it.next().unwrap().parse::<$t>().unwrap()
    };
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());
    let q = scan!(iter, usize);
    for _ in 0..q {
        if let (Some(s), Some(t)) = (iter.next(), iter.next()) {
            writeln!(
                writer,
                "{}",
                if t == "a" {
                    1
                } else if t.contains("a") {
                    -1
                } else {
                    1i64 << s.len()
                }
            )?;
        }
    }
    Ok(())
}
