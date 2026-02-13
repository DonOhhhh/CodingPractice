use std::io::{self, Read, Write, BufWriter};
use std::collections::HashMap;

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
    let mut map: HashMap<usize, usize> = HashMap::new();
    for _ in 0..scan!(iter, usize) {
        map.clear();
        let (_n, mut k) = (scan!(iter, usize), scan!(iter, usize));
        let s = iter.next().unwrap().as_bytes().to_vec();
        for (i, &f_ch) in s.iter().enumerate() {
            if map.contains_key(&i) {
                continue;
            }
            let target_ch = f_ch ^ 0x20;
            // writeln!(writer, "{} {}", f_ch as char, target_ch as char)?;
            for (j, &s_ch) in s.iter().enumerate() {
                if i == j || map.contains_key(&j) {
                    continue;
                }
                if target_ch == s_ch {
                    map.insert(i,j);
                    map.insert(j,i);
                    break;
                }
            }
            // writeln!(writer, "map len : {}", map.len() / 2)?;
        }
        // writeln!(writer, "[Round 2]")?;
        for (i, &f_ch) in s.iter().enumerate() {
            if map.contains_key(&i) {
                continue;
            }
            let target_ch = f_ch ^ 0x20;
            // writeln!(writer, "{} {}", f_ch as char, target_ch as char)?;
            for (j, &s_ch) in s.iter().enumerate() {
                if i == j || map.contains_key(&j) {
                    continue;
                }
                if s_ch == f_ch && k > 0 {
                    map.insert(i,j);
                    map.insert(j,i);
                    k -= 1;
                    break;
                }
            }
            // writeln!(writer, "map len : {}", map.len() / 2)?;
        }
        writeln!(writer, "{}", map.len() / 2)?;
    }
    Ok(())
}
