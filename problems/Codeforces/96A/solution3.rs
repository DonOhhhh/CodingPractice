use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let bytes = input.trim().as_bytes();
    
    // 길이 7짜리 슬라이딩 윈도우를 만듭니다.
    // 각 윈도우(w)가 "00..." 또는 "11..."과 같은지 확인합니다.
    let is_dangerous = bytes.windows(7).any(|w| w == b"0000000" || w == b"1111111");

    print!("{}", if is_dangerous { "YES" } else { "NO" });
    Ok(())
}