use std::collections::VecDeque;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim();

    let mut res = VecDeque::new();
    for (i, c) in s.chars().enumerate() {
        if (n - i) % 2 == 1 {
            res.push_back(c);
        } else {
            res.push_front(c);
        }
    }

    // 한 번에 출력
    let result: String = res.into_iter().collect();
    println!("{}", result);
}