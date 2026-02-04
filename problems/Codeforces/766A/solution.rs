use std::cmp::max;

fn main() {
    let mut lines = std::io::stdin().lines();
    let first = lines.next().unwrap().unwrap();
    let second = lines.next().unwrap().unwrap();
    if first == second {
        print!("-1");
    } else {
        print!("{}", max(first.len(), second.len()));
    }
}
