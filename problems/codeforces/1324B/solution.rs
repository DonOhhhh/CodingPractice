use std::io::{self, BufWriter, Read, Write};

macro_rules! scan {
    ($it: expr, $t:ty) => {
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
        let a_len = scan!(iter, usize);
        let mut pos: [usize; 5001] = [usize::MAX; 5001];
        let a: Vec<usize> = iter.by_ref().take(a_len).map(|e| e.parse().unwrap()).collect();
        let mut res = false;
        for i in 0..a_len {
            let num = a[i];
            if pos[num] == usize::MAX {
                pos[num] = i;
                continue;
            }
            if (i - pos[num]) >= 2 {
                res = true;
                break;
            }
        }
        writeln!(writer, "{}", if res { "YES" } else { "NO" })?;
    }
    Ok(())
}
