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
    for _ in 0..scan!(iter, usize) {
        let n = scan!(iter, usize);
        let s = iter.next().unwrap();
        let mut prefix_zero = s.bytes().take_while(|&e| e == b'0').count();
        let mut suffix_one = s.bytes().rev().take_while(|&e| e == b'1').count();
        for _ in 0..prefix_zero {
            write!(writer, "0")?;
        }

        if prefix_zero + suffix_one < n {
            write!(writer, "0")?;
        }

        for _ in 0..suffix_one {
            write!(writer, "1")?;
        }

        writeln!(writer)?;
    }
    Ok(())
}
