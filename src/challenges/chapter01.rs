use decode::technique::substitution::caesar;
use std::path::Path;

pub fn part_a(input_path: &Path) {
    println!("Running C01, Part A, using {:?}", input_path);
    let mut shift = caesar::Caesar::init(input_path);
    dbg!(&shift.key);
    let decoded = shift.decode();
    dbg!(&shift.key);
    println!("Decoded input: {}", decoded);
}
