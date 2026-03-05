use std::{env::args, io::{self, Read}};
use anyhow::Result;
use blake3::hash;

const ALGORITHM: &str = "BLAKE3";

// Board borders
const WIDTH: usize = 23;
const HEIGHT: usize = 12;
// Bishop

const CHARS: &str = ".o+=*BOX@%&#/^";

fn draw(data: &[u8]) -> Result<String> {
    
    let mut x = (WIDTH-1)/2;
    let mut y = (HEIGHT-1)/2;

    let mut painting = vec![0u8; WIDTH * HEIGHT];
    let start_pos = (x,y);
    for byte in data {
        for shift in (0..8).step_by(2) {
            // i dont really understand anything in there so i just pray that it works ehehe
            let mut bit = (byte >> shift) & 3;
            x = (x as i32 + ((bit & 1) as i32 * 2) - 1).clamp(0, (WIDTH-1) as i32) as usize;
            y = (y as i32 + (((bit >> 1) & 1) as i32 * 2) - 1).clamp(0, (HEIGHT-1) as i32) as usize;
            let idx = y * WIDTH + x;
            painting[idx] = painting[idx].saturating_add(1);
        }
    }    
    let mut output = String::with_capacity(((WIDTH + 3) * HEIGHT) + 4);
    
    output.push('+');
    output.push_str(&"-".repeat(WIDTH));
    output.push_str(r"+
");
    
    for row in (0..HEIGHT) {
        output.push('|');
        for col in (0..WIDTH) {
            let pos = (col,row);
            let idx = row * WIDTH + col;
            if pos == (x,y) {
                output.push('E');
            } else if pos == start_pos {
                output.push('S');
            } else {
                let count = painting[idx] as usize;
                let char_idx = count.min(CHARS.len() - 1);
                output.push(CHARS.as_bytes()[char_idx] as char);
            }
        }
        output.push_str(r"|
");
    }
    //ugly stuff
    let padding = WIDTH - ALGORITHM.len() - 2;
    output.push('+');
    output.push_str(&"-".repeat(padding/2));
    output.push('[');
    output.push_str(ALGORITHM);
    output.push(']');
    output.push_str(&"-".repeat(padding-padding/2));
    output.push('+');
    Ok(output)
}

fn main() -> Result<()> {
    let args: Vec<String> = args().skip(1).collect();
    let mut input = Vec::new();
    if !args.is_empty() {
        input = args.join(" ").into_bytes();
    } else {
        io::stdin().read_to_end(&mut input);
    }
    println!("{}", draw(
        hash(
            &input
        ).as_bytes()
    )?); 
    Ok(())
}
