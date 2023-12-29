use decode::technique::substitution::caesar;
use std::path::Path;

pub fn part_a(input_path: &Path) {
    println!("Running C01, Part A, using {:?}", input_path);
    let mut shift = match caesar::Caesar::init(input_path) {
        Ok(initialised) => initialised,
        Err(error) => panic!("Unable to initialise Caesar decoder {}", error)
    };
    dbg!(shift.key());
    let decoded = shift.decode();
    dbg!(shift.key());
    println!("Decoded input: {}", decoded);
}
