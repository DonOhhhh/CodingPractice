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
    let t = scan!(iter, usize);
    for _ in 0..t {
        let mut res = "NO";
        let mut count = 0;
        let s = iter.next().unwrap();
        for (i, ch) in s.bytes().enumerate() {
            if ch == b'(' {
                count += 1;
            } else {
                count -= 1;
            }
            if i != s.len() - 1 && count == 0 {
                res = "YES";
                break;
            }
        }
        writeln!(writer, "{}", res)?;
    }
    Ok(())
}
