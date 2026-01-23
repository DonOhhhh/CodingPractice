use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    let result = input
        .trim()              // 1. 입력 앞뒤의 공백/줄바꿈 제거
        .split("WUB")        // 2. "WUB"을 기준으로 문자열 조각내기
        .filter(|s| !s.is_empty()) // 3. 빈 문자열(WUB이 연속될 때 생김) 제거
        .collect::<Vec<_>>() // 4. 벡터로 모으기
        .join(" ");          // 5. 단어 사이에 공백 하나만 넣어서 합치기

    println!("{}", result);
    Ok(())
}