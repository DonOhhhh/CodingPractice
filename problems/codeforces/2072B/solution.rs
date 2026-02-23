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
    for _t in 0..t {
        let n = scan!(iter, usize);
        let underscore_count = iter.next().unwrap().chars().filter(|&ch| ch == '_').count();
        let hypen_count = n - underscore_count;
        let left_h = hypen_count / 2;
        let right_h = hypen_count - left_h;
        writeln!(writer, "{}", left_h * right_h * underscore_count)?;
    }
    Ok(())
}
