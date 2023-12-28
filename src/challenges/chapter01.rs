use decode::technique::substitution::caesar;
use std::path::Path;

pub fn part_a(input_path: &Path) {
    println!("Running C01, Part A, using {:?}", input_path);
    caesar::print_hello_world();
}
