use std::io::{self, BufWriter, Read, Write};

macro_rules! scan {
    ($it: expr, $t: ty) => {
        $it.next().unwrap().parse::<$t>().unwrap()
    };
}

fn eraseLetter(
    vec: &mut Vec<u8>
    , idx: usize
    , b_byte: u8
    , last_pos_vec: &mut Vec<usize>
) {
    if vec[idx] == b_byte {
        vec[idx] = b' ';
        if let Some(last_idx) = last_pos_vec.pop() {
            vec[last_idx] = b' ';
        }
    } else {
        last_pos_vec.push(idx);
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());
    let t = scan!(iter, usize);
    for _t in 0..t {
        let mut last_lowercase_pos = Vec::new();
        let mut last_uppercase_pos = Vec::new();

        let mut s_vec = iter.next().unwrap().as_bytes().to_vec();
        let s_len = s_vec.len();
        for i in 0..s_len {
            let byte = s_vec[i];
            // lowercase
            if byte & 0x20 != 0 {
                eraseLetter(&mut s_vec, i, b'b', &mut last_lowercase_pos);
                // if byte == b'b' {
                //     s_vec[i] = b' ';
                //     if let Some(last_idx) = last_lowercase_pos.pop() {
                //         s_vec[last_idx] = b' ';
                //     }
                //     continue;
                // }
                // last_lowercase_pos.push(i);
            // uppercase
            } else {
                eraseLetter(&mut s_vec, i, b'B', &mut last_uppercase_pos);
                // if byte == b'B' {
                //     s_vec[i] = b' ';
                //     if let Some(last_idx) = last_uppercase_pos.pop() {
                //         s_vec[last_idx] = b' ';
                //     }
                //     continue;
                // }
                // last_uppercase_pos.push(i);
            }
        }
        // let res: String = s_vec
        //     .iter()
        //     .filter(|&&e| e != b' ')
        //     .map(|&e| e as char)
        //     .collect();
        // writeln!(writer, "{}", res)?;
        for &byte in s_vec.iter().filter(|&&e| e != b' ') {
            writer.write_all(&[byte])?;
        }
        writer.write_all(b"\n")?;
    }
    Ok(())
}
