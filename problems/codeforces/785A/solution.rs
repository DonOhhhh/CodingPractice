use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;

    let count = input
        .split_ascii_whitespace()
        .skip(1)
        .fold(0, |acc, s| {
            let faces = match s.as_bytes()[0] {
                b'T' => 4,  // Tetrahedron
                b'C' => 6,  // Cube
                b'O' => 8,  // Octahedron
                b'D' => 12, // Dodecahedron
                b'I' => 20, // Icosahedron
                _ => 0,
            };
            acc + faces
        });

    println!("{count}");
    Ok(())
}
