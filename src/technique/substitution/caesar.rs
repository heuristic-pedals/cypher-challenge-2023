use std::collections::HashMap;
use std::fs::{read_to_string, self};
use std::io;
use std::path::Path;

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
        let input_bytes_iter: std::slice::Iter<'_, u8> = self._coded_input.as_bytes().iter();

        // 69 is the E char - calculate shift needed to map most common onto this
        let most_common_bytes = Caesar::most_common_uppercase_ascii(&input_bytes_iter);
        let key: i16 = 69 - most_common_bytes as i16;
        self.key = Some(key);
        let decoded_input = input_bytes_iter
            .map(|x| Caesar::map_char(*x, key))
            .collect::<Vec<char>>();

        let decoded_input = String::from_iter(decoded_input);
        self._decoded_input = Some(decoded_input.clone());
        decoded_input
    }

    fn most_common_uppercase_ascii(bytes_iter: &std::slice::Iter<'_, u8>) -> u8 {
        let input_bytes_chars = bytes_iter
            .clone()
            .filter(|x| x.is_ascii_uppercase())
            .collect::<Vec<&u8>>();

        let mut freqs: HashMap<u8, usize> = HashMap::new();
        for b in input_bytes_chars {
            let count = freqs.entry(*b).or_insert(0);
            *count += 1;
        }
        freqs.iter().max_by_key(|b| b.1).unwrap().0.clone()
    }

    fn map_char(coded_char: u8, key: i16) -> char {
        if !coded_char.is_ascii_uppercase() {
            return coded_char as char;
        }
        (((coded_char as i16 - 65 + key) % 26 + 26) % 26 + 65) as u8 as char
    }

    pub fn key(&self) -> Option<i16> {
        self.key
    }

    pub fn write_decoded_input(& self, output_path: &Path) -> Result<(), io::Error> {

        match &self._decoded_input {
            Some(decoded_input) => fs::write(output_path, decoded_input),
            None => {
                println!("An input has not been decoded. Call `init` and `decode` beforehand.");
                Ok(())
            }
        }
    }
}
