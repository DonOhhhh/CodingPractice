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
        let (mut a, mut b) = (scan!(iter, i32), scan!(iter, i32));
        let mut s: Vec<char> = iter.next().unwrap().chars().collect();
        let n = s.len();

        // 1. 이미 있는 문자들 개수 차감
        for &c in &s {
            if c == '0' {
                a -= 1;
            } else if c == '1' {
                b -= 1;
            }
        }

        // 2. 팰린드롬 대칭 맞추기 (한쪽만 ?인 경우)
        let mut possible = true;
        for i in 0..n / 2 {
            let j = n - 1 - i;
            if s[i] != '?' && s[j] == '?' {
                if s[i] == '0' {
                    a -= 1;
                    s[j] = '0';
                } else {
                    b -= 1;
                    s[j] = '1';
                }
            } else if s[i] == '?' && s[j] != '?' {
                if s[j] == '0' {
                    a -= 1;
                    s[i] = '0';
                } else {
                    b -= 1;
                    s[i] = '1';
                }
            }
        }

        // 3. 둘 다 ?인 자리 채우기 (2개씩 짝지어서)
        for i in 0..n / 2 {
            let j = n - 1 - i;
            if s[i] == '?' && s[j] == '?' {
                if a >= 2 {
                    s[i] = '0';
                    s[j] = '0';
                    a -= 2;
                } else if b >= 2 {
                    s[i] = '1';
                    s[j] = '1';
                    b -= 2;
                }
            }
        }

        // 4. 홀수 길이일 때 정중앙 처리
        if n % 2 == 1 && s[n / 2] == '?' {
            if a >= 1 {
                s[n / 2] = '0';
                a -= 1;
            } else if b >= 1 {
                s[n / 2] = '1';
                b -= 1;
            }
        }

        // 5. 최종 검증 (팰린드롬 여부 & 남은 개수 0인지)
        if a != 0 || b != 0 {
            possible = false;
        }
        for i in 0..n / 2 {
            if s[i] != s[n - 1 - i] || s[i] == '?' {
                possible = false;
            }
        }

        if possible {
            writeln!(writer, "{}", s.iter().collect::<String>())?;
        } else {
            writeln!(writer, "-1")?;
        }
    }
    Ok(())
}
