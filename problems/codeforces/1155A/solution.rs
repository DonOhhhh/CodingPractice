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
    let n = scan!(iter, usize);
    let s = iter.next().unwrap().as_bytes().to_vec();
    let mut res = false;
    let mut l = 0;
    let mut r = 0;
    for i in 0..n-1 {
        if s[i] > s[i+1] {
            res = true;
            l = i+1;
            r = i+2;
            break;
        }
    }

    if res {
        writeln!(writer, "YES")?;
        writeln!(writer, "{} {}", l, r)?;
    } else {
        writeln!(writer, "NO")?;
    }
    Ok(())
}
