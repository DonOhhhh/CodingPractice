use std::io::{self, BufWriter, Read, Write};
// itertools 크레이트에서 기능과 열거형을 가져옵니다.
use itertools::{Itertools, EitherOrBoth::Both};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    
    let out = io::stdout();
    let mut writer = BufWriter::new(out.lock());
    
    if let Some(t_str) = iter.next() {
        let t = t_str.parse::<usize>().unwrap();
        for _ in 0..t {
            let p = iter.next().unwrap().as_bytes();
            let s = iter.next().unwrap().as_bytes();
            
            // 💡 여기서부터 진짜 원라이너 시작!
            let res = p.chunk_by(|a, b| a == b)
                .map(|chunk| (chunk[0], chunk.len()))
                .zip_longest(s.chunk_by(|a, b| a == b).map(|chunk| (chunk[0], chunk.len())))
                .all(|pair| {
                    // Both(p의 블록, s의 블록) 형태일 때만 검사를 통과시킵니다.
                    if let Both((p_char, p_cnt), (s_char, s_cnt)) = pair {
                        p_char == s_char && s_cnt >= p_cnt && s_cnt <= p_cnt * 2
                    } else {
                        false // 길이가 달라서 한쪽만 남은 경우(Left/Right) 무조건 실패
                    }
                });
            
            writeln!(writer, "{}", if res { "YES" } else { "NO" })?;
        }
    }
    Ok(())
}