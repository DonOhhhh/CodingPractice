use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    // 1. WUB -> 공백으로 치환 ("  ABC DEF")
    // 2. split_ascii_whitespace -> 공백 덩어리 무시하고 단어만 추출 (["ABC", "DEF"])
    // 3. join -> 다시 합치기 ("ABC DEF")
    let result = input.replace("WUB", " ")
        .split_ascii_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", result);
    Ok(())
}