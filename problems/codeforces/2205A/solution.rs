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
    let mut v = Vec::with_capacity(500);
    for _ in 0..scan!(iter, usize) {
        let n = scan!(iter, usize);
        v.clear();
        for _ in 0..n {
            v.push(scan!(iter, usize));
        }
        let mut max_val = 0;
        let mut max_i = 0;
        for i in 0..n {
            if v[i] > max_val {
                max_val = v[i];
                max_i = i;
            }
        }
        (v[0], v[max_i]) = (v[max_i], v[0]);
        writeln!(writer, "{}", v.iter().map(|&e| format!("{} ", e)).collect::<String>().trim())?;
    }
    Ok(())
}
