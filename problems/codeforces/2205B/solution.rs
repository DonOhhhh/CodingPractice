use std::io::{self, BufWriter, Read, Write};

macro_rules! scan {
    ($it: expr, $t: ty) => {
        $it.next().unwrap().parse::<$t>().unwrap()
    };
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());
    for _ in 0..scan!(iter, usize) {
        let mut n = scan!(iter, u64);
        let mut k = 1u64; // 정답이 될 k (초기값은 곱셈을 위해 1)

        // 1. 소인수분해 시작: 2부터 n의 제곱근까지만 확인합니다.
        let mut d = 2u64;
        while d * d <= n {
            // d가 n의 약수(소인수)라면?
            if n % d == 0 {
                k *= d; // 정답 k에 이 소인수를 딱 1번만 곱해줍니다.

                // n에서 d라는 소인수를 완전히 쥐어짜서 없애버립니다 (중복 방지)
                while n % d == 0 {
                    n /= d;
                }
            }
            d += 1;
        }

        // 2. 루프가 끝났는데도 n이 1보다 크다면?
        // 그 남은 n 자체가 아주 큰 '소수(Prime Number)'라는 뜻입니다.
        if n > 1 {
            k *= n; // 마지막 남은 소인수도 정답 k에 곱해줍니다.
        }

        writeln!(writer, "{}", k)?;
    }
    Ok(())
}
