use std::io::{self, Read, Write, BufWriter};
use std::collections::HashSet;

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
    let mut set: HashSet<String> = HashSet::with_capacity(200000);
    
    let t = scan!(iter, usize);
    for _t in 0..t {
        let _n = scan!(iter, usize);
        set.clear();
        let s_vec = iter.next().unwrap();
        let s_len = s_vec.len();
        for i in 0..s_len-1 {
            let possible_str: String = s_vec[0..i].to_owned() + &s_vec[i+2..s_len];
            // writeln!(writer, "[{}][{}][{}]", &s_vec[0..i], &s_vec[i..i+2], &s_vec[i+2..s_len])?;
            set.insert(possible_str);
        }
        writeln!(writer, "{}", set.len())?;
    }
    Ok(())
}
