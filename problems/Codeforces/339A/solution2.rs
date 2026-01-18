use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // Read all input from stdin
    let mut buffer = String::with_capacity(128);
    io::stdin().read_to_string(&mut buffer)?;

    // Use Counting Sort (O(N)) which is faster than Heap Sort (O(N log N)) 
    // or standard Sort (O(N log N)) for a small set of values (1, 2, 3).
    let mut count1 = 0;
    let mut count2 = 0;
    let mut count3 = 0;

    for &byte in buffer.as_bytes() {
        match byte {
            b'1' => count1 += 1,
            b'2' => count2 += 1,
            b'3' => count3 += 1,
            _ => {} // Ignore '+' and newline
        }
    }

    let total = count1 + count2 + count3;
    if total == 0 {
        return Ok(());
    }

    // pre-allocate exact size: N digits + N separators (we will ignore the last one)
    let mut result = Vec::with_capacity(total * 2);

    // Helper to fill the vector: ALWAYS push digit then '+'
    let mut fill = |count: usize, byte: u8| {
        for _ in 0..count {
            result.push(byte);
            result.push(b'+');
        }
    };

    fill(count1, b'1');
    fill(count2, b'2');
    fill(count3, b'3');

    // Write everything except the last trailing '+'
    let mut out = io::stdout().lock();
    // We are guaranteed total > 0 here, so result is not empty.
    out.write_all(&result[..result.len() - 1])?;
    Ok(())
}
