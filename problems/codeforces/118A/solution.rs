use std::io::{self, Read, Write, BufWriter};

fn main() -> io::Result<()> {
    let mut input = String::with_capacity(1 << 7);
    io::stdin().lock().read_to_string(&mut input)?;

    // 'a', 'e', 'i', 'o', 'u', 'y' | 1, 5, 9, 15, 21, 25
    let check = 0u32 | (1 << 0) | (1 << 4) | (1 << 8) | (1 << 14) | (1 << 20) | (1 << 24);
    let mut output : Vec<u8> = Vec::with_capacity(1 << 7);
    for &c in input.trim().as_bytes() {
        let lower_c = c | 0x20;
        if (check >> (lower_c - b'a') as i8) & 1 == 1 { continue; }
        output.push(b'.');
        output.push(lower_c);
    }

    let mut writer = BufWriter::new(io::stdout().lock());
    writer.write_all(&mut output)?;
    Ok(())
}
