use std::io::{self, Read, Write, BufWriter};

fn main() -> io::Result<()> {
    const CAPACITY: usize = 1 << 13;
    let mut input = String::with_capacity(CAPACITY);
    io::stdin().lock().read_to_string(&mut input)?;
    let mut output = String::with_capacity(CAPACITY);

    for s in input.split_ascii_whitespace().skip(1) {
        let mut zeros = 0;
        let mut ones = 0;
        let s_bytes = s.as_bytes();
        for &c in s_bytes {
            if c == b'0' {
                zeros += 1;
            } else {
                ones += 1;
            }
        }

        let mut ans = 0;
        for i in 0..s.len() {
            if s_bytes[i] == b'0' {
                if ones > 0 {
                    ones -= 1;
                } else {
                    ans = s.len() - i;
                    break;
                }
            } else {
                if zeros > 0 {
                    zeros -= 1;
                } else {
                    ans = s.len() - i;
                    break;
                }
            }
        }
        output.push_str(&format!("{}\n", ans));
    }
    let mut writer = BufWriter::with_capacity(CAPACITY, io::stdout().lock());
    writer.write_all(output.as_bytes())?;
    Ok(())
}
