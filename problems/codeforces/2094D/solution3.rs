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
            
            // 💡 마법의 1줄: chunk_by로 묶고, map으로 (문자, 길이) 튜플로 변환!
            let mut p_blocks = p.chunk_by(|a, b| a == b).map(|chunk| (chunk[0], chunk.len()));
            let mut s_blocks = s.chunk_by(|a, b| a == b).map(|chunk| (chunk[0], chunk.len()));
            
            let mut res = true;
            
            // 검증 로직은 이전과 동일하게 패턴 매칭으로 깔끔하게 처리
            loop {
                match (p_blocks.next(), s_blocks.next()) {
                    (Some((p_char, p_cnt)), Some((s_char, s_cnt))) => {
                        if p_char != s_char || s_cnt < p_cnt || s_cnt > p_cnt * 2 {
                            res = false;
                            break;
                        }
                    }
                    (None, None) => break, // 둘 다 무사히 끝남
                    _ => {
                        res = false; // 한쪽이 먼저 끝남 (블록 개수 다름)
                        break;
                    }
                }
            }
            
            writeln!(writer, "{}", if res { "YES" } else { "NO" })?;
        }
    }
    Ok(())
}