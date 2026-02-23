use std::io::{self, BufWriter, Read, Write};

macro_rules! scan {
    ($it: expr, $t: ty) => {
        $it.next().unwrap().parse::<$t>().unwrap()
    };
}

// writer를 인자로 받아 같은 버퍼에 쓰도록 수정
fn print_matrix<W: Write>(writer: &mut W, matrix: &Vec<Vec<i32>>) -> io::Result<()> {
    for v in matrix {
        writeln!(writer, "{:?}", v)?;
    }
    writeln!(writer)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());
    for _ in 0..scan!(iter, usize) {
        let (n, q) = (scan!(iter, usize), scan!(iter, usize));
        let (a, b) = (
            iter.next().unwrap().as_bytes().to_vec(),
            iter.next().unwrap().as_bytes().to_vec(),
        );
        let mut pref_a = vec![vec![0i32; n + 1]; 26];
        let mut pref_b = vec![vec![0i32; n + 1]; 26];
        
        for i in 0..n {
            let char_a = (a[i] - b'a') as usize;
            let char_b = (b[i] - b'a') as usize;
            for c in 0..26 {
                pref_a[c][i + 1] = pref_a[c][i];
                pref_b[c][i + 1] = pref_b[c][i];
            }
            pref_a[char_a][i + 1] += 1;
            pref_b[char_b][i + 1] += 1;
        }

        for _ in 0..q {
            let (l, r) = (scan!(iter, usize), scan!(iter, usize));
            let mut diff_sum = 0;
            for c in 0..26 {
                let count_a = pref_a[c][r] - pref_a[c][l-1];
                let count_b = pref_b[c][r] - pref_b[c][l-1];
                if count_a > count_b {
                    diff_sum += count_a - count_b;
                }
            }
            writeln!(writer, "{}", diff_sum)?;
        }
    }
    Ok(())
}
