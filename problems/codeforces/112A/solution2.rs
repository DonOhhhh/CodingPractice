use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    let mut words = input.split_ascii_whitespace();
    let s1 = words.next().unwrap().as_bytes();
    let s2 = words.next().unwrap().as_bytes();

    let mut output = 0;
    for i in 0..s1.len() {
        let c1 = s1[i] | 0x20;
        let c2 = s2[i] | 0x20;
        if c1 < c2 {
            output = -1;
            break;
        } else if c1 > c2 {
            output = 1;
            break;
        }
    }
    
    print!("{output}");
    Ok(())
}