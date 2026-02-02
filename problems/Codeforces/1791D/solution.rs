use std::io::{self, Read, Write, BufWriter};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());
    let t: usize = iter.next().unwrap().parse().unwrap();
    for _ in 0..t {
        iter.next();
        let s = iter.next().unwrap().as_bytes();
        let mut left_freq = [0; 26];
        let mut right_freq = [0; 26];
        let mut left_distinct = 0;
        let mut right_distinct = 0;
        let mut max_cnt = 0;
        for &c in s {
            let idx = (c - b'a') as usize;
            if right_freq[idx] == 0 {
                right_distinct += 1;
            }
            right_freq[idx] += 1;
        }

        for &c in s {
            let idx = (c - b'a') as usize;
            if left_freq[idx] == 0 {
                left_distinct += 1;
            }
            left_freq[idx] += 1;

            right_freq[idx] -= 1;
            if right_freq[idx] == 0 {
                right_distinct -= 1;
            }

            max_cnt = max_cnt.max(left_distinct + right_distinct);
        }
        writeln!(writer, "{}", max_cnt)?;
    }
    Ok(())
}
