use std::io::{self, BufWriter, Read, Write};

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
            
            // 💡 표준 라이브러리만 사용한 궁극의 체이닝!
            let res = p.chunk_by(|a, b| a == b)
                .map(|chunk| (chunk[0], chunk.len()))
                // eq_by: 길이가 다르면 즉시 false, 같으면 아래의 클로저로 쌍을 비교
                .eq_by(
                    s.chunk_by(|a, b| a == b).map(|chunk| (chunk[0], chunk.len())),
                    |(p_char, p_cnt), (s_char, s_cnt)| {
                        p_char == s_char && s_cnt >= p_cnt && s_cnt <= p_cnt * 2
                    }
                );
            
            writeln!(writer, "{}", if res { "YES" } else { "NO" })?;
        }
    }
    Ok(())
}