use std::io::{self, Read, Write, BufWriter};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());
    let t = iter.next().unwrap().parse().unwrap();
    let mut output = Vec::new();
    for _ in 0..t {
        let n: isize = iter.next().unwrap().parse().unwrap();
        let s = iter.next().unwrap().as_bytes();
        let mut i = n - 1;
        while i > 0 {
            if s[i as usize] == b'a' || s[i as usize] == b'e' {
                output.push(s[i as usize]);
                output.push(s[(i-1) as usize]);
                i-=2;
            } else {
                output.push(s[i as usize]);
                output.push(s[(i-1) as usize]);
                output.push(s[(i-2) as usize]);
                i-=3;
            }
            output.push(b'.')
        }
        output.pop();
        output.reverse();
        output.push(b'\n');
        writer.write_all(&mut output)?;
        output.clear();
    }
    Ok(())
}
