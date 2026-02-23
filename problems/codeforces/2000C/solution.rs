use std::collections::HashMap;
use std::io::{self, BufWriter, Read, Write};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());
    let t = iter.next().unwrap().parse().unwrap();

    let mut char_to_num: Vec<Option<i32>> = vec![None; 26];
    let mut num_to_char: HashMap<i32, u8> = HashMap::new();

    for _t in 0..t {
        let a_len = iter.next().unwrap().parse().unwrap();
        let a: Vec<i32> = iter
            .by_ref()
            .take(a_len)
            .map(|s| s.parse().unwrap())
            .collect();
        // let mut a = Vec::<i32>::with_capacity(a_len);
        // for _a_len in 0..a_len {
        //     a.push(iter.next().unwrap().parse::<i32>().unwrap());
        // }
        let m = iter.next().unwrap().parse().unwrap();
        for _m in 0..m {
            let s = iter.next().unwrap();
            if s.len() != a_len {
                writeln!(writer, "NO")?;
                continue;
            }

            let mut is_ok = "YES";
            char_to_num.fill(None);
            num_to_char.clear();

            for (i, byte) in s.bytes().enumerate() {
                let idx = (byte - b'a') as usize;
                if let Some(num) = char_to_num[idx] {
                    if num != a[i] {
                        is_ok = "NO";
                        break;
                    }
                } else {
                    if num_to_char.contains_key(&a[i]) {
                        is_ok = "NO";
                        break;
                    }

                    char_to_num[idx] = Some(a[i]);
                    num_to_char.insert(a[i], byte);
                }
            }
            writeln!(writer, "{}", is_ok)?;
        }
    }
    Ok(())
}
