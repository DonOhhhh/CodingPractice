use std::io::{self, Read, Write, BufWriter};
use std::collections::HashMap;

macro_rules! scan {
    ($it:expr, $t:ty) => {
        $it.next().unwrap().parse::<$t>().unwrap()
    };
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());
    let mut canonical_int_map: HashMap<i32, i32> = HashMap::new();
    let mut canonical_char_map: [i32; 26] = [-1; 26];
    let mut a_vec = Vec::with_capacity(200_005);

    let t: i32 = scan!(iter, i32);
    for _t in 0..t {
        let n = scan!(iter, usize);

        canonical_int_map.clear();
        a_vec.clear();
        let mut next_id_for_num: i32 = 0;

        for e in iter.by_ref().take(n) {
            let num: i32 = e.parse().unwrap();
            let id = *canonical_int_map.entry(num).or_insert_with(|| {
                let val = next_id_for_num;
                next_id_for_num += 1;
                val
            });
            a_vec.push(id);
        }

        let m = scan!(iter, usize);
        for _m in 0..m {
            let s = iter.next().unwrap();
            if n != s.len() {
                writeln!(writer, "NO")?;
                continue;
            }
            let mut next_id_for_char: i32 = 0;
            let mut res = "YES";

            canonical_char_map.fill(-1);
            for (i, byte) in s.bytes().enumerate() {
                let idx = (byte - b'a') as usize;
                if canonical_char_map[idx] == -1 {
                    canonical_char_map[idx] = next_id_for_char;
                    next_id_for_char += 1;
                }
                if canonical_char_map[idx] != a_vec[i] {
                    res = "NO";
                    break;
                }
            }
            writeln!(writer, "{}", res)?;
        }
    }

    Ok(())
}