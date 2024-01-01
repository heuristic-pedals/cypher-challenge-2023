use std::collections::HashMap;
use std::fs::{self, create_dir_all, read_to_string};
use std::io;
use std::path::Path;
use std::str::Chars;

#[derive(Debug)]
pub struct Caesar {
    _coded_input: String,
    _decoded_input: Option<String>,
    key: Option<i16>,
}

impl Caesar {
    pub fn init(input_path: &Path) -> Result<Caesar, io::Error> {
        let input = read_to_string(input_path)?;
        Ok(Caesar {
            _coded_input: input,
            _decoded_input: None,
            key: None,
        })
    }

    pub fn decode(&mut self) -> String {
        // a common iterator - set as variable to only create this once
        let input_chars: Chars<'_> = self._coded_input.chars();

        // 69 is the E char - calculate shift needed to map most common onto this
        let most_common_char = Caesar::most_common_uppercase_ascii(&input_chars);
        let key: i16 = 69 - most_common_char as i16;
        self.key = Some(key);
        let decoded_input = input_chars
            .map(|x| Caesar::map_char(x, key))
            .collect::<Vec<char>>();

        let decoded_input = String::from_iter(decoded_input);
        self._decoded_input = Some(decoded_input.clone());
        decoded_input
    }

    fn most_common_uppercase_ascii(input_chars: &Chars<'_>) -> u8 {
        let input_bytes_chars = input_chars
            .clone()
            .filter(|x| x.is_ascii_uppercase())
            .collect::<Vec<char>>();

        let mut freqs: HashMap<char, usize> = HashMap::new();
        for b in input_bytes_chars {
            let count = freqs.entry(b).or_insert(0);
            *count += 1;
        }
        freqs.iter().max_by_key(|b| b.1).unwrap().0.clone() as u8
    }

    fn map_char(coded: char, key: i16) -> char {
        if !coded.is_ascii_uppercase() {
            return coded as char;
        }
        (((coded as i16 - 65 + key) % 26 + 26) % 26 + 65) as u8 as char
    }

    pub fn key(&self) -> Option<i16> {
        self.key
    }

    pub fn write_decoded_input(&self, output_path: &Path) -> Result<(), io::Error> {
        let parent_dir = match &output_path.parent() {
            Some(dir) => dir,
            None => Path::new("/"),
        };
        if !parent_dir.exists() {
            create_dir_all(parent_dir)?;
        }

        match &self._decoded_input {
            Some(decoded_input) => fs::write(output_path, decoded_input),
            None => {
                println!("An input has not been decoded. Call `init` and `decode` beforehand.");
                Ok(())
            }
        }
    }
}
