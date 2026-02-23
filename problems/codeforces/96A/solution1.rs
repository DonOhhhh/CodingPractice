use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::with_capacity(128);
    io::stdin().lock().read_to_string(&mut input)?;
    
    let mut count = 0;
    let mut last = 0; // 직전 문자 저장

    for b in input.trim().bytes() {
        if b == last {
            count += 1;
        } else {
            count = 1;
            last = b;
        }

        // 7개가 되는 순간 바로 종료 (Early Return)
        if count >= 7 {
            print!("YES");
            return Ok(());
        }
    }

    print!("NO");
    Ok(())
}