use std::io::{self, BufWriter, Read, Write};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());
    let (s, t) = (
        iter.next().unwrap().as_bytes().to_vec(),
        iter.next().unwrap().as_bytes().to_vec(),
    );
    let (s_len, t_len) = (s.len(), t.len());
    let mut count = s_len.min(t_len);
    for i in 1..=count {
        if s[s_len - i] != t[t_len - i] {
            count = i - 1;
            break;
        }
    }
    writeln!(writer, "{}", s_len + t_len - 2 * count)?;
    Ok(())
}
