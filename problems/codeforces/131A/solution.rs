use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut input = String::with_capacity(128);
    io::stdin().lock().read_to_string(&mut input)?;
    let mut s = input.trim().to_string().into_bytes();
    if s.iter().skip(1).all(|c| c.is_ascii_uppercase()) {
        for c in s.iter_mut() {
            *c ^= 32;
        }
    }
    io::stdout().lock().write_all(&s)?;
    Ok(())
}