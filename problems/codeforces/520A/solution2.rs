use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    let s = input.split_ascii_whitespace().nth(1).unwrap();

    let mut count = 0u32;
    for b in s.bytes() {
        count |= 1 << ((b | 0x20) - b'a');
    }

    print!("{}", if count == (1 << 26) - 1 { "YES" } else { "NO" });
    Ok(())
}