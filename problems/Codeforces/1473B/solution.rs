use std::io::{self, BufWriter, Read, Write};

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let t = iter.next().unwrap().parse().unwrap();
    let mut writer = BufWriter::new(io::stdout().lock());
    for _ in 0..t {
        let s = iter.next().unwrap();
        let t = iter.next().unwrap();

        let n = s.len();
        let m = t.len();
        let l = lcm(n, m);

        let res_s = s.repeat(l / n);
        let res_t = t.repeat(l / m);
        writeln!(writer, "{}", if res_s == res_t { &res_s } else { "-1" })?;
    }
    Ok(())
}
