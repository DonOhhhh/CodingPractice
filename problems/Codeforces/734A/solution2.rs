use std::io::{self, Read};
use std::cmp::Ordering;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    let s = input.split_ascii_whitespace().nth(1).unwrap();
    
    // score가 양수면 Anton, 음수면 Danik, 0이면 Friendship
    let score: i32 = s.bytes().fold(0, |acc, b| acc + if b == b'A' { 1 } else { -1 });

    print!("{}", match score.cmp(&0) {
        Ordering::Greater => "Anton",
        Ordering::Less    => "Danik",
        Ordering::Equal   => "Friendship",
    });
    Ok(())
}