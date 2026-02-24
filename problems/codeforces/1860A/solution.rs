use std::io::{self, BufWriter, Read, Write};

macro_rules! scan {
    ($it: expr, $t: ty) => {
        $it.next().unwrap().parse::<$t>().unwrap()
    };
}

fn solve<W: Write>(w: &mut W, s: &str) -> io::Result<()> {
    if s == "()" {
        writeln!(w, "NO")?;
        return Ok(());
    }
    let n = s.len();
    let s1 = "(".repeat(n) + &")".repeat(n);
    let s2 = "()".repeat(n);

    if !s1.contains(s) {
        writeln!(w, "YES")?;
        writeln!(w, "{}", s1)?;
    } else if !s2.contains(s) {
        writeln!(w, "YES")?;
        writeln!(w, "{}", s2)?;
    } else {
        writeln!(w, "NO")?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());
    for _ in 0..scan!(iter, usize) {
        let s = iter.next().unwrap();
        solve(&mut writer, s)?;
    }
    Ok(())
}
