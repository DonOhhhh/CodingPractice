use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    let v: Vec<u8> = input.trim_end().bytes().map(|b| b | 0x20).collect();
    let mut len = v.len() / 2 - 1;
    let mut s2_start = len + 2;

    let mut output = 0;
    for i in 0..len {
        if v[i] < v[s2_start + i] {
            output = -1;
            break;
        } else if v[i] > v[s2_start + i] {
            output = 1;
            break;
        }
    }
    print!("{output}");
    Ok(())
}
