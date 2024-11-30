use core::panic;

use crate::read_utf::ReadUTF;

pub struct ReadUTFDelims {
    pub read_utf: ReadUTF,
}

impl ReadUTFDelims {
    pub fn new(
        path: String,
        delimiter: Vec<String>,
        print_invalid_char: Option<bool>,
        buffer_size: Option<usize>,
    ) -> Result<ReadUTFDelims, std::io::Error> {

        let read = match ReadUTF::new(path, Some(delimiter), print_invalid_char, buffer_size) {
            Ok(res) => res,
            _ => {panic!("Unable to create ReadUTFDelims")}
        };

        Ok(ReadUTFDelims {
            read_utf: read,
        })
    }
}

impl Iterator for ReadUTFDelims {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read_utf.read_delim() {
            Ok(b) => {
                if b {
                    return Some(self.read_utf.line.to_string());
                }
            },
            _ => {}
        }
        self.read_utf.close();
        None
    }
}