use std::io::{self, BufWriter, Read, Write};

// 문자열을 (문자, 연속된 개수) 형태의 블록 배열로 압축하는 헬퍼 함수
fn get_blocks(s: &[u8]) -> Vec<(u8, usize)> {
    let mut blocks = Vec::new();
    if s.is_empty() { return blocks; }
    
    let mut curr_char = s[0];
    let mut count = 1;
    
    for &ch in &s[1..] {
        if ch == curr_char {
            count += 1; // 같은 글자면 카운트 증가
        } else {
            blocks.push((curr_char, count)); // 다른 글자가 나오면 블록 저장
            curr_char = ch;
            count = 1;
        }
    }
    blocks.push((curr_char, count)); // 마지막 블록 저장
    blocks
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    
    // stdout을 잠그고 BufWriter를 생성하여 출력 속도 극대화
    let out = io::stdout();
    let mut writer = BufWriter::new(out.lock());
    
    if let Some(t_str) = iter.next() {
        let t = t_str.parse::<usize>().unwrap();
        for _ in 0..t {
            // to_vec()으로 복사하지 않고, as_bytes() 슬라이스 자체를 바로 넘김!
            let p = iter.next().unwrap().as_bytes();
            let s = iter.next().unwrap().as_bytes();
            
            let p_blocks = get_blocks(p);
            let s_blocks = get_blocks(s);
            
            let mut res = true;
            
            // 1. 블록 개수가 다르면 무조건 실패
            if p_blocks.len() != s_blocks.len() {
                res = false;
            } else {
                // 2. 각 블록을 1:1로 비교
                for i in 0..p_blocks.len() {
                    let (p_char, p_cnt) = p_blocks[i];
                    let (s_char, s_cnt) = s_blocks[i];
                    
                    // 문자가 다르거나, s가 더 짧거나, s가 p의 2배보다 길면 실패
                    if p_char != s_char || s_cnt < p_cnt || s_cnt > p_cnt * 2 {
                        res = false;
                        break;
                    }
                }
            }
            
            writeln!(writer, "{}", if res { "YES" } else { "NO" })?;
        }
    }
    Ok(())
}