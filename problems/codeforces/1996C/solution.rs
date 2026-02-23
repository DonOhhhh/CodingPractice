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
        let (_n, q) = (scan!(iter, usize), scan!(iter, usize));
        let (a, b) = (
            iter.next().unwrap().as_bytes().to_vec(),
            iter.next().unwrap().as_bytes().to_vec(),
        );
        for _q in 0..q {
            let mut count: [isize; 26] = [0; 26];
            let (l, r) = (scan!(iter, usize) - 1, scan!(iter, usize) - 1);
            for i in l..=r {
                let mut idx = (a[i] - b'a') as usize;
                count[idx] += 1;
                idx = (b[i] - b'a') as usize;
                count[idx] -= 1;
            }
            writeln!(writer, "{}", count.iter().filter(|&&e| e > 0).sum::<isize>())?;
        }
    }
    Ok(())
}
