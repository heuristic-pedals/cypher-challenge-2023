use std::io;
use std::path::Path;
use std::fs::read_to_string;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Caesar {
    _coded_input: String,
    key: Option<i16>,
}

impl Caesar {
    pub fn init(input_path: &Path) -> Result<Caesar, io::Error> {
        let input = read_to_string(input_path)?;
        Ok(Caesar { _coded_input: input,key: None })
    }
    pub fn decode(&mut self) -> String {
        let input_bytes = self._coded_input.as_bytes()
            .iter()
            .filter(|b| **b >= 65 as u8 && **b <= 90)
            .collect::<Vec<&u8>>();

        let mut freqs: HashMap<u8, usize> = HashMap::new();
        for b in input_bytes {
            let count = freqs.entry(*b).or_insert(0);
            *count += 1;
        }
        let most_common_bytes = freqs.iter().max_by_key(|b| b.1).unwrap().0;
        let key: i16 = 69 - *most_common_bytes as i16;
        self.key = Some(key);

        let decoded_input = self._coded_input.as_bytes()
                .iter()
                .filter(|x| x.is_ascii())
                .map(|b| if *b >= 65 && *b <= 90 {((*b as i16 - 65 + key) % 26 + 26) % 26 + 65} else { *b as i16 })
                .map(|i| i as u8 as char)
                .collect::<Vec<char>>();

        String::from_iter(decoded_input)
    }
    pub fn key(& self) -> Option<i16> {
        self.key
    }
}
