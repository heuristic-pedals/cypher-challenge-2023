mod challenges;
use argparse::{ArgumentParser, Store};
use std::path::Path;
use std::process::exit;

// enumerate valid chapter parts
enum Part {
    A,
    B,
}

fn main() {
    // alloc variables for parsing inputs
    let mut chapter = String::new();
    let mut part = String::new(); // set part A as default
    let mut input_path = String::new();
    let mut output_path = String::new();
    const VALID_PARTS: [&str; 2] = ["A", "B"]; // created for print statement

    {
        // this scope is used to limit borrows of `ap.refer` - see argparse docs
        let mut ap = ArgumentParser::new();
        ap.set_description("Cypher Challenge 2023");
        ap.refer(&mut chapter)
            .add_argument("chapter", Store, "Chapter number to execute.")
            .required();
        ap.refer(&mut part)
            .add_argument("part", Store, "Part of chapter to run.")
            .required();
        ap.refer(&mut input_path)
            .add_argument("input_path", Store, "Path to input file.")
            .required();
        ap.refer(&mut output_path)
            .add_argument("output_path", Store, "Path to write decoded output.")
            .required();
        ap.parse_args_or_exit();
    }

    // handle cases; unable to parse chapter, invalid part, or non-existant input_path
    let chapter_number = match chapter.parse::<u8>() {
        Ok(num) => num,
        Err(_) => {
            println!("Unable to parse chapter to a number.");
            exit(0);
        }
    };
    let chapter_part = match part.as_str() {
        "A" | "a" => Part::A,
        "B" | "b" => Part::B,
        _ => {
            println!(
                "Unexpected value for `part`: {}. Should be one of {:?}.",
                part, VALID_PARTS
            );
            exit(0);
        }
    };
    let input = match Path::new(&input_path).exists() {
        true => Path::new(&input_path),
        _ => {
            println!("Unable to find `input_path` {}", input_path);
            exit(0);
        }
    };
    let output = Path::new(&output_path);

    // execute requested options
    match (chapter_number, chapter_part) {
        (1, Part::A) => challenges::chapter01::part_a(input, output),
        _ => {
            println!("Unexpected value for `chapter`: {}.", chapter);
            exit(0);
        }
    }
}
