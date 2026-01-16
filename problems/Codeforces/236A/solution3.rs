use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines();
    if let Some(Ok(s)) = lines.next() {
        let mut chars: Vec<char> = s.trim().chars().collect();
        chars.sort_unstable();
        chars.dedup();

        if chars.len() % 2 == 0 {
            print!("CHAT WITH HER!");
        } else {
            print!("IGNORE HIM!");
        }
    }
}