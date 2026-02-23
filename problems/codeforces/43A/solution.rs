use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let iter = input.split_ascii_whitespace().skip(1);

    let mut leader = "";
    let mut score = 0;

    // Boyer-Moore Voting Algorithm
    for team in iter {
        if score == 0 {
            leader = team;
            score = 1;
        } else if team == leader {
            score += 1;
        } else {
            score -= 1;
        }
    }

    print!("{}", leader);
    Ok(())
}
