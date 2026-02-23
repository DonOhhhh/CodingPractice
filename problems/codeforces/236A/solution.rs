use std::io::{self, *};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut input = Vec::with_capacity(100);
    io::stdin().lock().read_to_end(&mut input)?;
    let mut set: HashSet<u8> = HashSet::new();

    for &c in input.iter() {
        if (c >= b'a' && c <= b'z') && !set.contains(&c) {
            set.insert(c);
        }
    }

    if set.len() % 2 == 0 {
        println!("CHAT WITH HER!")
    } else {
        print!("IGNORE HIM!")
    }
    Ok(())
}
