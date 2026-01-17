use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    let mut anton = 0;
    let mut danik = 0;
    let s = input.split_ascii_whitespace().skip(1).next().unwrap();
    for c in s.chars() {
        if c == 'A' {
            anton += 1; 
        } else {
            danik += 1;
        }
    }

    let result;
    if anton > danik {
        result = "Anton";
    } else if anton < danik {
        result = "Danik";
    } else {
        result = "Friendship";
    }
    print!("{result}");
    Ok(())
}
