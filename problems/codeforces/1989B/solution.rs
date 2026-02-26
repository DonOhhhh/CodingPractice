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
        let a = iter.next().unwrap().as_bytes();
        let n = a.len();
        let b = iter.next().unwrap().as_bytes();
        let m = b.len();
        let mut max_overlap = 0;

        // b의 모든 시작 지점 i를 시도합니다.
        for i in 0..m {
            let mut b_idx = i;
            let mut overlap = 0;

            // a를 순회하면서 b[b_idx..]의 문자들이 부분 수열로 존재하는지 확인
            for &char_a in a {
                if b_idx < m && char_a == b[b_idx] {
                    b_idx += 1;
                    overlap += 1;
                }
            }

            // 가장 많이 겹치는(부분 수열로 존재하는) 길이를 갱신
            if overlap > max_overlap {
                max_overlap = overlap;
            }
        }

        // 결과 = a의 길이 + (b의 길이 - 겹치는 길이)
        writeln!(writer, "{}", n + m - max_overlap)?;
    }
    Ok(())
}
