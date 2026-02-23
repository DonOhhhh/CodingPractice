use std::io::{self, Read, Write, BufWriter};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());
    let k: usize = iter.next().unwrap().parse().unwrap();
    let s: &str = iter.next().unwrap();
    let mut count = [0usize; 26];
    for &c in s.as_bytes() {
        count[(c-b'a') as usize] += 1;
    }

    for &n in &count {
        if n % k != 0 {
            writeln!(writer, "-1")?;
            return Ok(());
        }
    }

    let mut pattern: Vec<u8> = Vec::with_capacity(s.len() / k);
    for (i, &n) in count.iter().enumerate() {
        if n == 0 {
            continue;
        }
        for _ in 0..(n/k) {
            pattern.push(b'a' + i as u8);
        }
    }
    for _ in 0..k {
        writer.write_all(&pattern)?;
    }
    writeln!(writer)?;
    Ok(())
}