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
    let t = scan!(iter, usize);
    for _ in 0..t {
        let n = scan!(iter, usize);
        let mut v: Vec<usize> = Vec::with_capacity(n+1);
        v.push(0);
        for _ in 0..n {
            v.push(scan!(iter, usize));
        }
        let mut res = true;
        for i in 1..n+1 {
            let mut odd_i = i;
            while odd_i % 2 == 0 {
                odd_i /= 2;
            }
            let mut odd_val = v[i];
            while odd_val % 2 == 0 {
                odd_val /= 2; 
            }
            if odd_i != odd_val {
                res = false;
                break;
            }
        }
        writeln!(writer, "{}", if res {"YES"} else {"NO"})?;
    }
    Ok(())
}
