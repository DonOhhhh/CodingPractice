use std::io::{self, BufWriter, Read, Write};

macro_rules! scan {
    ($it: expr, $t: ty) => {
        $it.next().unwrap().parse::<$t>().unwrap()
    };
}

fn solve(s: &str) {
    let bytes = s.as_bytes();
    let n = bytes.len();

    let mut found_ten_plus = false;

    // 1. 뒤에서부터 탐색하여 합이 10 이상인 경우 찾기
    // (n-2)부터 0까지 역순으로 조사
    for i in (0..n - 1).rev() {
        let a = bytes[i] - b'0';
        let b = bytes[i + 1] - b'0';
        let sum = a + b;

        if sum >= 10 {
            // [앞부분] + [합] + [뒷부분]
            let prefix = &s[..i];
            let suffix = &s[i + 2..];
            println!("{}{}{}", prefix, sum, suffix);
            found_ten_plus = true;
            break;
        }
    }

    // 2. 10 이상인 경우가 없었다면 무조건 맨 앞 두 글자를 합침
    if !found_ten_plus {
        let sum = (bytes[0] - b'0') + (bytes[1] - b'0');
        let suffix = &s[2..];
        println!("{}{}", sum, suffix);
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());
    for _ in 0..scan!(iter, usize) {
        let x = iter.next().unwrap();
        solve(&x);
    }
    Ok(())
}
