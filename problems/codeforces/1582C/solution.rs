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
        let s = iter.next().unwrap().as_bytes();
        let mut res = n + 1;
        for c in b'a'..=b'z' {
            let mut cnt = 0;
            let mut l = 0;
            let mut r = n - 1;
            while l < r {
                if s[l] == s[r] {
                    l += 1;
                    r -= 1;
                } else if s[l] == c {
                    cnt += 1;
                    l += 1;
                } else if s[r] == c {
                    cnt += 1;
                    r -= 1;
                } else {
                    cnt = n + 1;
                    break;
                }
            }
            res = res.min(cnt);
        }
        writeln!(writer, "{}", if res == n + 1 {-1} else {res as i32})?;
    }
    Ok(())
}
