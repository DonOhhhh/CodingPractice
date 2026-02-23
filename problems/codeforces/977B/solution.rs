use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    // 1. 입력을 한 번에 다 읽습니다. (가장 빠른 I/O)
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    
    // 2. 공백 기준으로 토큰을 나눕니다. 
    // 첫 번째 토큰(N)은 무시하고, 두 번째 토큰(문자열 S)을 바이트로 가져옵니다.
    let mut words = buffer.split_ascii_whitespace();
    words.next(); // N은 필요 없으므로 건너뜀
    let s = words.next().unwrap().as_bytes();

    // 3. 빈도수 계산
    let mut counts = HashMap::new();
    for w in s.windows(2) {
        // &[u8] 슬라이스를 키로 사용하므로 String 복사가 발생하지 않음!
        *counts.entry(w).or_insert(0) += 1;
    }

    // 4. 최댓값 찾기 (O(N))
    let (best_pair, _) = counts.iter()
        .max_by_key(|&(_, count)| count) // count가 가장 큰 항목을 찾음
        .unwrap();

    // 5. 결과 출력
    println!("{}", std::str::from_utf8(best_pair).unwrap());
}