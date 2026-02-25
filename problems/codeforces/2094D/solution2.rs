use std::io::{self, BufWriter, Read, Write};

// 1. 상태를 저장할 뼈대(구조체)를 만듭니다.
// 원본 바이트 슬라이스의 참조(&[u8])만 들고 있습니다.
struct Blocks<'a> {
    slice: &'a [u8],
}

// 2. Iterator 트레이트를 구현하여 "다음 블록"을 찾는 로직을 작성합니다.
impl<'a> Iterator for Blocks<'a> {
    type Item = (u8, usize); // 반환할 타입: (문자, 개수)

    fn next(&mut self) -> Option<Self::Item> {
        // 남은 슬라이스가 없으면 반복 종료
        if self.slice.is_empty() {
            return None;
        }

        let ch = self.slice[0];
        let mut count = 1;

        // 현재 문자와 같은 문자가 연속으로 몇 개 있는지 셉니다.
        while count < self.slice.len() && self.slice[count] == ch {
            count += 1;
        }

        // 핵심: 방금 읽은 만큼 슬라이스의 시작점을 앞으로 당깁니다! (데이터 복사 없음)
        self.slice = &self.slice[count..];

        Some((ch, count))
    }
}

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
            
            // Vec 할당 없이 단순히 슬라이스를 구조체에 넘겨 반복자를 생성합니다.
            let mut p_blocks = Blocks { slice: p };
            let mut s_blocks = Blocks { slice: s };
            
            let mut res = true;
            
            // 3. 두 반복자를 동시에 하나씩 뽑아서 비교합니다.
            loop {
                match (p_blocks.next(), s_blocks.next()) {
                    (Some((p_char, p_cnt)), Some((s_char, s_cnt))) => {
                        // 문자가 다르거나, 조건에 맞지 않으면 즉시 실패
                        if p_char != s_char || s_cnt < p_cnt || s_cnt > p_cnt * 2 {
                            res = false;
                            break;
                        }
                    }
                    (None, None) => {
                        // 둘 다 깔끔하게 동시에 끝났다면 성공
                        break;
                    }
                    _ => {
                        // 어느 한쪽이 먼저 끝나버렸다면 (블록 개수가 다름) 실패
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