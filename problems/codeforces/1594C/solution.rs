use std::io::{self, Read, Write, BufWriter};

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
        let n = scan!(iter, usize);
        let c = scan!(iter, char);
        let s = iter.next().unwrap();
        let s_bytes = s.as_bytes();
        
        // 1단계: 이미 모든 문자가 c인지 확인
        // .iter().all()을 사용하면 도중에 틀린 게 나올 시 바로 멈춥니다 (Short-circuit)
        if s_bytes.iter().all(|&b| b == c as u8) {
            writeln!(writer, "0")?;
            continue;
        }

        // 2단계: 1번의 연산으로 가능한 x 찾기
        let mut found_x = None;
        // x는 1부터 n까지 가능합니다.
        for x in 1..=n {
            let mut possible = true;
            // x의 배수 자리를 모두 검사 (x, 2x, 3x...)
            // step_by를 사용하여 효율적으로 건너뜁니다.
            for j in (x..=n).step_by(x) {
                if s_bytes[j - 1] != c as u8 {
                    possible = false;
                    break;
                }
            }

            if possible {
                found_x = Some(x);
                break;
            }
        }

        if let Some(x) = found_x {
            writeln!(writer, "1\n{}", x)?;
        } else {
            // 3단계: 1번으로 안 되면 무조건 n과 n-1 두 번으로 해결
            writeln!(writer, "2\n{} {}", n - 1, n)?;
        }
    }
    Ok(())
}
