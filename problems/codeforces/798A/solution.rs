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
    let s = iter.next().unwrap();
    let s_vec = s.as_bytes().to_vec();
    let n = s.len();
    let mut count = 0;
    for i in 0..n/2 {
        let j = n - 1 - i;
        if s_vec[i] != s_vec[j] {
            count += 1;
        }
    }
    writeln!(writer, "{}", if count == 1 || (n % 2 == 1 && count == 0) {"YES"} else {"NO"})?;
    Ok(())
}
