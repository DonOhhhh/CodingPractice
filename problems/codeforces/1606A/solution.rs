use std::io::{self, Read, Write, BufWriter};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());
    let t: i32 = iter.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut chars: Vec<char> = iter.next().unwrap().chars().collect();
        let n = chars.len();
        if chars[0] != chars[n-1] {
            chars[0] = chars[n-1];
        }
        let res: String = chars.into_iter().collect();
        writeln!(writer, "{}", res)?;
    }
    Ok(())
}
