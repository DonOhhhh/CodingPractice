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
        let (_n, mut k) = (scan!(iter, usize), scan!(iter, usize));
        let mut upper_count: [isize; 26] = [0; 26];
        let mut lower_count: [isize; 26] = [0; 26];
        let mut res = 0;
        for &b in iter.next().unwrap().as_bytes() {
            if (b & 0x20) != 0 {
                lower_count[(b - b'a') as usize] += 1;
            } else {
                upper_count[(b - b'A') as usize] += 1;
            }
        }
        for i in 0..26 {
            res += lower_count[i].min(upper_count[i]);
            let mut unpaired = (lower_count[i] - upper_count[i]).abs();
            while unpaired > 1 && k > 0 {
                res += 1;
                k -= 1;
                unpaired -= 2;
            }
        }
        writeln!(writer, "{}", res)?;
    }
    Ok(())
}
