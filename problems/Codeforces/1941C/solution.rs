use std::io::{self, BufWriter, Read, Write};

fn main() -> io::Result<()> {
    const CAPACITY: usize = 1 << 20;
    let mut input = String::with_capacity(CAPACITY);
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let t = iter.next().unwrap().parse().unwrap();
    let mut writer = BufWriter::new(io::stdout().lock());
    for _ in 0..t {
        let n = iter.next().unwrap().parse().unwrap();
        let s = iter.next().unwrap();
        let mut ans = 0;
        let mut i = 0;
        while i < n {
            if i + 4 < n && &s[i..i + 5] == "mapie" {
                ans += 1;
                i += 5;
            } else if i + 2 < n 
                        && (
                            &s[i..i + 3] == "map" 
                            || &s[i..i + 3] == "pie"
                        ) {
                ans += 1;
                i += 3;
            } else {
                i += 1;
            }
        }
        writeln!(writer, "{}", ans)?;
    }
    Ok(())
}
