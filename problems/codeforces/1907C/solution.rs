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
        let n = scan!(iter, i32);
        let s = iter.next().unwrap().as_bytes();
        let mut cnt: [usize; 26] = [0; 26];
        let mut most_cnt: i32 = 0;
        for &b in s {
            let idx = (b - b'a') as usize;
            cnt[idx] += 1;
            most_cnt = most_cnt.max(cnt[idx] as i32);
        }
        writeln!(writer, "{}", (n % 2).max(2 * most_cnt - n))?;
    }
    Ok(())
}
