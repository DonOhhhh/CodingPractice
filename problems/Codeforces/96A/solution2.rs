use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // contains는 내부적으로 매우 최적화된(SIMD 등) 문자열 탐색을 수행합니다.
    if input.contains("0000000") || input.contains("1111111") {
        print!("YES");
    } else {
        print!("NO");
    }
    Ok(())
}