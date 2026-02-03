use std::io::{self, Read, Write, BufWriter};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());
    let t = iter.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let n = iter.next().unwrap().parse::<usize>().unwrap();
        let mut count = [0u32; 26];
        let mut s = Vec::<u8>::new();
        for __ in 0..n {
            let a = iter.next().unwrap().parse::<u32>().unwrap();
            for (i, c) in count.iter_mut().enumerate() {
                if *c == a {
                    s.push(b'a' + i as u8);
                    *c += 1;
                    break;
                }
            }
        }
        s.push(b'\n');
        writer.write_all(&mut s)?;
    }
    Ok(())
}
