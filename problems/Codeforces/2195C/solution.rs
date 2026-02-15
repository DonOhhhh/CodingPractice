use std::io::{self, BufWriter, Read, Write};
use std::cmp;

// 입력을 빠르게 받기 위한 매크로
macro_rules! scan {
    ($it: expr, $t: ty) => {
        $it.next().unwrap().parse::<$t>().unwrap()
    };
}

// 주사위의 두 면이 인접한지 확인하는 함수
#[inline(always)]
fn is_adjacent(u: usize, v: usize) -> bool {
    u != v && u + v != 7
}

fn solve() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());

    let t = scan!(iter, usize);

    for _ in 0..t {
        let n = scan!(iter, usize);
        
        // dp[v] : 현재 단계에서 값이 v로 끝날 때, 변경하지 않고 유지한 최대 개수
        let mut dp = [0; 7]; 

        // 첫 번째 원소 처리 (이전 원소가 없으므로 초기화)
        let first_val = scan!(iter, usize);
        for v in 1..=6 {
            if v == first_val {
                dp[v] = 1;
            } else {
                dp[v] = 0;
            }
        }

        // 두 번째 원소부터 끝까지 처리
        for _ in 1..n {
            let val = scan!(iter, usize);
            let mut next_dp = [0; 7];

            // 이번 위치에 올 수 있는 모든 주사위 눈(1~6)에 대해
            for curr in 1..=6 {
                let bonus = if curr == val { 1 } else { 0 };
                let mut max_prev = 0;

                // 이전 위치의 주사위 눈(1~6)을 확인
                for prev in 1..=6 {
                    if is_adjacent(curr, prev) {
                        if dp[prev] > max_prev {
                            max_prev = dp[prev];
                        }
                    }
                }
                next_dp[curr] = max_prev + bonus;
            }
            dp = next_dp;
        }

        // 전체 길이 n에서 (최대 유지 개수)를 빼면 최소 변경 횟수가 됨
        let max_kept = dp.iter().max().unwrap();
        writeln!(writer, "{}", n - max_kept)?;
    }

    Ok(())
}

fn main() {
    solve().unwrap();
}