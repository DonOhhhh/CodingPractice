use std::io::{self, Read, Write, BufWriter};
use std::collections::HashMap;

macro_rules! scan {
    ($it:expr, $t:ty) => {
        $it.next().unwrap().parse::<$t>().unwrap()
    };
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());
    let t = scan!(iter, i32);
    let mut numtochar: HashMap<i32, char> = HashMap::new();
    let mut chartonum: HashMap<char, i32> = HashMap::new();
    for _t in 0..t {
        let n = scan!(iter, usize);
        let mut a = Vec::with_capacity(n);
        for _n in 0..n {
            a.push(scan!(iter, i32));
        }
        let m = scan!(iter, i32);
        for _m in 0..m {
            let s: Vec<char> = iter.next().unwrap().chars().collect();
            if a.len() != s.len() {
                writeln!(writer, "NO")?;
                continue;
            }
            let mut res = true;
            numtochar.clear();
            chartonum.clear();
            for i in 0..a.len() {
                let ch: char = s[i];
                if chartonum.contains_key(&ch) != numtochar.contains_key(&a[i]) {
                    res = false;
                }
                if !chartonum.contains_key(&ch) {
                    chartonum.insert(ch, a[i]);
                    numtochar.insert(a[i], ch);
                } else if *chartonum.get(&ch).unwrap() != a[i] || 
                        *numtochar.get(&a[i]).unwrap() != ch {
                    res = false;
                }
            }
            writeln!(writer, "{}", if res {"YES"} else {"NO"})?;
        }
    }
    Ok(())
}