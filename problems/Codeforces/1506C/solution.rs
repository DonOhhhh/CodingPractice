use std::io::{self, Read, Write, BufWriter};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());
    let t: usize = iter.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut a: &str = iter.next().unwrap();
        let mut b: &str = iter.next().unwrap();
        let mut count: usize = 0;
        // 더 짧은 쪽 문자열을 b에 넣는다.
        if a.len() < b.len() {
            (a,b) = (b,a);
        }
        // 더 긴쪽 문자열(a)의 부분 문자열이 더 짧은 쪽 문자열(b)에 존재하는지 검사한다.
        for i in 0..a.len() {
            for j in i+1..a.len()+1 {
                if b.contains(&a[i..j]) && count < (j-i) {
                    count = j-i;
                }
            }
        }
        writeln!(writer, "{}", a.len() + b.len() - count * 2)?;
    }
    Ok(())
}
