use decode::technique::substitution::caesar;
use std::path::Path;

pub fn part_a(input_path: &Path, output_path: &Path, print_to_screen: bool) {
    println!("Running C01, Part A, using {:?}", input_path);
    let mut shift = match caesar::Caesar::new(input_path) {
        Ok(initialised) => initialised,
        Err(error) => panic!("Unable to initialise Caesar decoder {}", error),
    };

    let decoded = shift.decode();
    if print_to_screen {
        println!("Decoded Input: {}", decoded);
    }

    match shift.write_decoded_input(&output_path) {
        Ok(()) => {
            println!("Decoded input saved: {:?}", output_path);
            ()
        }
        Err(error) => panic!("IO error when writing decoded input: {error}"),
    }
}
