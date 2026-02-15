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
    for _ in 0..t {
        let n = scan!(iter, usize);
        let mut found_67 = false;
        for _ in 0..n {
            let a = scan!(iter, usize);
            if a == 67 {
                found_67 = true;
            }
        }
        writeln!(writer, "{}", if found_67 {"YES"} else {"NO"})?;
    }
    Ok(())
}
