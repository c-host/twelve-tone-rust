fn transpose(pitch: &u32, t: u32) -> u32 {
    (pitch + t) % 12
}

fn get_transposition(t: u32) -> Vec<u32> {
    let twelve_tone_row: Vec<u32> = (0..12).collect();
    let transposed_row: Vec<u32> = twelve_tone_row
        .iter()
        .map(|pitch| transpose(pitch, t))
        .collect();
    transposed_row
}

fn main() {
    for x in 0..12 {
        println!("{:?}", get_transposition(x));
    }
    // In reverse
    println!("In reverse\n");
    for x in (0..12).rev() {
        println!("{:?}", get_transposition(x));
    }
}

// TODO: Add tests.
