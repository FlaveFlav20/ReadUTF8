use core::panic;

use crate::read_utf8::ReadUTF8;

pub struct ReadUTF8Char {
    pub read_utf: ReadUTF8,
}

impl ReadUTF8Char {
    pub fn new(
        path: String,
        print_invalid_char: Option<bool>,
        buffer_size: Option<usize>,
    ) -> Result<ReadUTF8Char, std::io::Error> {
        let read = match ReadUTF8::new(path, None, print_invalid_char, buffer_size) {
            Ok(res) => res,
            _ => {
                panic!("Unable to create ReadUTF8Char")
            }
        };

        Ok(ReadUTF8Char { read_utf: read })
    }
}

impl Iterator for ReadUTF8Char {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read_utf.read_char() {
            Ok(b) => {
                if b {
                    return Some(self.read_utf.line.to_string());
                }
            }
            _ => {}
        }
        self.read_utf.close();
        None
    }
}
