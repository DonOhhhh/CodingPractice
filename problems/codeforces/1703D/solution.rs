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
    let t = scan!(iter, usize);
    for _t in 0..t {
        let n = scan!(iter, usize);
        let strings: Vec<&str> = iter.by_ref().take(n).collect();
        let mut res: Vec<u8> = Vec::with_capacity(n);
        for (i, &s_i) in strings.iter().enumerate() {
            let mut temp_res = false;
            for (j, &s_j) in strings.iter().enumerate() {
                if j == i {
                    continue;
                }
                for (k, &s_k) in strings.iter().enumerate() {
                    if k == i {
                        continue;
                    }
                    // writeln!(writer, "{} {} {}", s_i, s_j, s_k)?;
                    if s_i == (format!("{s_j}{s_k}")) 
                    || s_i == (format!("{s_k}{s_j}")) {
                        temp_res = true;
                        break;
                    }
                }
                if temp_res {
                    break;
                }
            }
            res.push(if temp_res { b'1' } else { b'0' });
        }
        writer.write_all(&mut res)?;
        writer.write_all(b"\n")?;
    }
    Ok(())
}
