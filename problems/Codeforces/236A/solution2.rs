use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut buf = [0u8; 128]; 
    let n = io::stdin().read(&mut buf)?;
    
    let mut mask: u32 = 0;
    
    for i in 0..n {
        let c = buf[i];
        if c >= b'a' && c <= b'z' {
            mask |= 1 << (c - b'a');
        }
    }
    
    let mut handle = io::stdout().lock();
    if mask.count_ones() % 2 == 0 {
        handle.write_all(b"CHAT WITH HER!")?;
    } else {
        handle.write_all(b"IGNORE HIM!")?;
    }

    Ok(())
}