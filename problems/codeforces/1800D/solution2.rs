use std::io::{self, Read, Write, BufWriter};

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
    for _t in 0..t {
        let _n = scan!(iter, usize);
        let s_vec = iter.next().unwrap().as_bytes().to_vec();
        let s_len = s_vec.len();
        let mut ans = s_len - 1;
        for i in 0..s_len-2 {
            if s_vec[i] == s_vec[i+2] {
                ans -= 1;
            }
        }
        writeln!(writer, "{}", ans)?;
    }
    Ok(())
}
