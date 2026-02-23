use std::io::{self, Read, Write, BufWriter};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());

    let t: usize = iter.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();
        let s: &[u8] = iter.next().unwrap().as_bytes();
        let mut count = [0; 26];
        let mut sum = 0;
        for (i, &ch) in s.iter().enumerate() {
            let idx = (ch - b'a') as usize;
            if count[idx] != 0 {
                continue;
            }
            sum += n - i;
            count[idx] = 1;
        }
        writeln!(writer, "{}", sum)?;
    }
    
    Ok(())
}
