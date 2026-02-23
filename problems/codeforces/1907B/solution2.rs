use std::io::{self, Read, Write, BufWriter};

fn eraseLetter(
    vec: &mut Vec<u8>
    , idx: usize
    , b_byte: u8
    , last_pos_vec: &mut Vec<usize>
) {
    if vec[idx] == b_byte {
        vec[idx] = b' ';
        if let Some(last_idx) = last_pos_vec.pop() {
            vec[last_idx] = b' ';
        }
    } else {
        last_pos_vec.push(idx);
    }
}

fn main() -> io::Result<()> {
    // 1. 전체 입력을 바이트 벡터 하나에 통째로 읽기
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;

    // 2. 공백/줄바꿈 기준으로 데이터를 쪼개는 반복자 생성
    let mut iter = buffer.split(|&b| b == b' ' || b == b'\n' || b == b'\r')
                         .filter(|chunk| !chunk.is_empty());

    let mut writer = BufWriter::new(io::stdout().lock());

    // 첫 번째 값(T) 읽기
    if let Some(t_bytes) = iter.next() {
        let t: usize = std::str::from_utf8(t_bytes).unwrap().parse().unwrap();
        
        let mut lower_stack = Vec::with_capacity(1000000);
        let mut upper_stack = Vec::with_capacity(1000000);

        for _ in 0..t {
            lower_stack.clear();
            upper_stack.clear();

            // s_vec를 새로 만들지 않고 슬라이스에서 가변 벡터로 복사
            let mut s_vec = iter.next().unwrap().to_vec();

            for i in 0..s_vec.len() {
                let byte = s_vec[i];
                if byte & 0x20 != 0 {
                    eraseLetter(&mut s_vec, i, b'b', &mut lower_stack);
                } else {
                    // 대문자 처리 로직
                    eraseLetter(&mut s_vec, i, b'B', &mut upper_stack);
                }
            }

            // 직접 출력
            for &b in &s_vec {
                if b != b' ' {
                    writer.write_all(&[b])?;
                }
            }
            writer.write_all(b"\n")?;
        }
    }
    writer.flush()?;
    Ok(())
}