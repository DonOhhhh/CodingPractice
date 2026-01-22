use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut words = input.split_ascii_whitespace();
    // as_bytes()를 사용하여 바이트 슬라이스로 가져옵니다.
    let s = words.next().unwrap().as_bytes();
    let t = words.next().unwrap().as_bytes();

    // bytes() 반복자로 비교
    if s.iter().rev().eq(t.iter()) {
        print!("YES");
    } else {
        print!("NO");
    }
}