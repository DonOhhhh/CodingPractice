use std::io::{self, BufWriter, Read, Write};
use std::collections::HashSet;

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
    let mut set: HashSet<&str> = HashSet::with_capacity(100_000);
    for _t in 0..t {
        let n = scan!(iter, usize);
        let mut ans: Vec<u8> = Vec::with_capacity(n);
        set.clear();
        let strs: Vec<&str> = iter.by_ref().take(n).map(|e| {set.insert(e); e}).collect();
        for &s in &strs {
            let mut is_divisible = false;
            for i in 1..s.len() {
                if set.contains(&s[0..i]) && set.contains(&s[i..]) {
                    is_divisible = true;
                    break;
                }
            }
            ans.push(if is_divisible {b'1'} else {b'0'});
        }
        writer.write_all(&mut ans)?;
        writer.write_all(b"\n")?;
    }
    Ok(())
}
