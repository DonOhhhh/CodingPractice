use std::io::{self, Read, Write, BufWriter};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());
    let t = iter.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut s = iter.next().unwrap().as_bytes().to_vec();
        let t = iter.next().unwrap().as_bytes();
        let mut cur = 0;
        for c in s.iter_mut() {
            if cur < t.len() {
                if *c == t[cur] || *c == b'?' {
                    *c = t[cur];
                    cur+=1;
                }
            } else {
                if *c == b'?' {
                    *c = b'a';
                }
            }
        }
        if cur == t.len() {
            writeln!(writer, "YES")?;
            writeln!(writer, "{}", str::from_utf8(&s).unwrap())?;
        } else {
            writeln!(writer, "NO")?;
        }
    }
    Ok(())
}
