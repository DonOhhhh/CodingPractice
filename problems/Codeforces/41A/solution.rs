use std::io::{self, BufRead};

fn main() {
    let mut v = Vec::<String>::new();
    for line in io::stdin().lock().lines() {
        v.push(line.unwrap());
    }
    print!("{}", if v.get(0).unwrap().chars().rev().collect::<String>() == *v.get(1).unwrap() {"YES"} else {"NO"});
}
