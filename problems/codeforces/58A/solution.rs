use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = Vec::new();
    io::stdin().lock().read_to_end(&mut input)?;
    let hello = [b'h',b'e',b'l',b'l',b'o'];
    let mut i = 0;
    for &c in &input {
        if i == 5 {
            break;
        }
        if c == hello[i] {
            i += 1;
        }
    }
    print!("{}", if i == 5 {"YES"} else {"NO"});
    Ok(())
}
