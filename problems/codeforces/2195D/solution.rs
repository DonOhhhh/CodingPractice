use std::io::{self, BufWriter, Read, Write};

// 입력을 편하게 받기 위한 매크로
macro_rules! scan {
    ($it: expr, $t: ty) => {
        $it.next().unwrap().parse::<$t>().unwrap()
    };
}

fn solve() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());

    let t = scan!(iter, usize);

    for _ in 0..t {
        let n = scan!(iter, usize);
        let mut f = Vec::with_capacity(n);
        for _ in 0..n {
            f.push(scan!(iter, i64));
        }

        let mut a = vec![0i64; n];

        // 1. 전체 합(S) 구하기
        // f(1) + f(n) = (n-1) * sum(a)
        let total_sum = (f[0] + f[n - 1]) / (n as i64 - 1);

        // 2. 첫 번째 원소 (a_1)
        // 2*a_1 = f(2) - f(1) + S
        a[0] = (f[1] - f[0] + total_sum) / 2;

        // 3. 마지막 원소 (a_n)
        // 2*a_n = S - (f(n) - f(n-1))
        a[n - 1] = (total_sum - (f[n - 1] - f[n - 2])) / 2;

        // 4. 중간 원소들 (a_2 ~ a_{n-1})
        // 2*a_i = f(i+1) - 2f(i) + f(i-1)
        // 인덱스 주의: 문제의 i는 1-based, 코드의 f는 0-based
        // a[i]를 구할 때 f[i+1], f[i], f[i-1]을 씀 (코드상 i는 a의 인덱스)
        // 실제로는 a[k] = (f[k+1] - 2f[k] + f[k-1]) / 2
        for k in 1..n - 1 {
            a[k] = (f[k + 1] - 2 * f[k] + f[k - 1]) / 2;
        }

        // 출력
        for i in 0..n {
            write!(writer, "{} ", a[i])?;
        }
        writeln!(writer)?;
    }

    Ok(())
}

fn main() {
    solve().unwrap();
}