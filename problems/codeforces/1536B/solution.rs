use std::io::{self, BufWriter, Read, Write};
use std::collections::HashSet;

macro_rules! scan {
    ($it: expr, $t: ty) => {
        $it.next().unwrap().parse::<$t>().unwrap()
    };
}

fn solve(s : &str, n: usize) -> String {
    let mut seen = HashSet::new();

    // 1. 모든 부분 문자열(길이 1, 2, 3)을 단 한 번의 순회로 수집
    for i in 0..n {
        // 길이 1
        seen.insert(&s[i..i + 1]);
        // 길이 2
        if i + 2 <= n {
            seen.insert(&s[i..i + 2]);
        }
        // 길이 3
        if i + 3 <= n {
            seen.insert(&s[i..i + 3]);
        }
    }

    let alphabet = "abcdefghijklmnopqrstuvwxyz".as_bytes();

    // 2. 사전 순 탐색 (상수 시간 연산)
    // 길이 1
    for &c1 in alphabet {
        let buf = [c1];
        let target = std::str::from_utf8(&buf).unwrap();
        if !seen.contains(target) {
            return target.to_string();
        }
    }

    // 길이 2
    for &c1 in alphabet {
        for &c2 in alphabet {
            let buf = [c1, c2];
            let target = std::str::from_utf8(&buf).unwrap();
            if !seen.contains(target) {
                return target.to_string();
            }
        }
    }

    // 길이 3
    for &c1 in alphabet {
        for &c2 in alphabet {
            for &c3 in alphabet {
                let buf = [c1, c2, c3];
                let target = std::str::from_utf8(&buf).unwrap();
                if !seen.contains(target) {
                    return target.to_string();
                }
            }
        }
    }

    "".to_string()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());
    for _ in 0..scan!(iter, usize) {
        let n = scan!(iter, usize);
        let s = iter.next().unwrap();
        writeln!(writer, "{}", solve(s, n))?;
    }
    Ok(())
}
