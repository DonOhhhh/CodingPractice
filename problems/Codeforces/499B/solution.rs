use std::collections::HashMap;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // 1. Fast I/O 설정
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut output = io::BufWriter::new(io::stdout().lock());

    // 2. 모든 입력을 공백/줄바꿈 기준으로 쪼개는 단일 이터레이터 생성
    let mut iter = input.split_ascii_whitespace();

    // 3. N, M 파싱 (N은 사실 반복문에서 안 쓰여서 무시 가능하지만 파싱은 해야 함)
    let _n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    // 4. HashMap 생성 (용량을 미리 지정하여 재할당 방지)
    let mut dict = HashMap::with_capacity(m);

    for _ in 0..m {
        // 이터레이터에서 두 단어씩 가져오기
        let u = iter.next().unwrap();
        let v = iter.next().unwrap();

        // 5. 더 짧은 단어 선택 로직 (길이가 같으면 첫 번째 단어 u)
        let shorter = if v.len() < u.len() { v } else { u };
        
        // 문제 조건상 강연에는 '첫 번째 언어'의 단어만 나옵니다.
        // 따라서 u를 key로, 선택된 shorter를 value로 저장하면 됩니다.
        dict.insert(u, shorter);
    }

    // 6. 남은 토큰(강연 내용) 처리 및 출력
    for word in iter {
        // 맵에서 찾아서 바로 버퍼에 씀 (String 할당 제거)
        write!(output, "{} ", dict[word])?;
    }

    Ok(())
}