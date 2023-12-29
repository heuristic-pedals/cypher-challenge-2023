use std::path::Path;
use std::fs::read_to_string;

#[derive(Debug)]
pub struct Caesar {
    _coded_input: String,
    pub key: Option<u8>,
}

impl Caesar {
    pub fn init(input_path: &Path) -> Caesar {
        let input = match read_to_string(input_path) {
            Ok(contents) => contents,
            Err(error) => {
                panic!("An error occured when using `input_path`: {}", error)
            }
        };
        Caesar {
            _coded_input: input,
            key: None,
        }
    }
    pub fn decode(&mut self) -> String {
        println!("Decoding a caesar cypher...");
        self.key = Some(10);
        String::from("test")
    }
}
