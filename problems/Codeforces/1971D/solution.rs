use std::io::{self, Read, Write, BufWriter};

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
    for _t in 0..t {
        let s = iter.next().unwrap().as_bytes().to_vec();
        let mut count = 1;
        let mut chance = false;
        for i in 1..s.len() {
            if s[i-1] == b'1' && s[i] == b'0' {
                count += 1;
            }
            if s[i-1] == b'0' && s[i] == b'1' {
                if chance {
                    count += 1;
                } else {
                    chance = true;
                }
            }
        }
        writeln!(writer, "{}", count)?;
    }
    Ok(())
}
