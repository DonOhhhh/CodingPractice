use std::io::{self, Read, Write, BufWriter};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());

    let k = iter.next().unwrap().parse().unwrap();
    let s = iter.next().unwrap();

    let mut count = [0usize; 26];
    for &c in s.as_bytes() {
        count[(c - b'a') as usize]+=1;
    }
    let flag = count.iter().all(|&e| e % k == 0);
    if !flag {
        writeln!(writer, "-1")?;
    } else {
        let kstring: String = count
                .iter()
                .enumerate()
                .map(|(i, &n)| {
                    if n == 0 {
                        return "".to_string();
                    }
                    let character = (b'a' + i as u8) as char;
                    character.to_string().repeat(n / k)
                }).collect();
        writeln!(writer, "{}", kstring.repeat(k))?;
    }
    Ok(())
}
