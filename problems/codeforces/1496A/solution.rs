use std::io::{self, BufWriter, Read, Write};

macro_rules! scan {
    ($it: expr, $t: ty) => {
        $it.next().unwrap().parse::<$t>().unwrap()
    };
    ($it: expr) => {
        $it.next().unwrap()
    };
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());
    for _ in 0..scan!(iter, usize) {
        let n = scan!(iter, usize);
        let k = scan!(iter, usize);
        let s = scan!(iter).as_bytes();
        // s의 앞 k글자와 뒤 k글자를 뒤집은 것이 일치하는지 확인
        let is_symmetric = s
            .iter()
            .take(k)
            .zip(s.iter().rev().take(k))
            .all(|(a, b)| a == b);

        if 2 * k < n && is_symmetric {
            writeln!(writer, "YES")?;
        } else {
            writeln!(writer, "NO")?;
        }
    }
    Ok(())
}
