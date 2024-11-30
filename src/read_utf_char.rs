use core::panic;

use crate::read_utf::ReadUTF;

pub struct ReadUTFChar {
    pub read_utf: ReadUTF,
}

impl ReadUTFChar {
    pub fn new(
        path: String,
        print_invalid_char: Option<bool>,
        buffer_size: Option<usize>,
    ) -> Result<ReadUTFChar, std::io::Error> {
        let read = match ReadUTF::new(path, None, print_invalid_char, buffer_size) {
            Ok(res) => res,
            _ => {
                panic!("Unable to create ReadUTFChar")
            }
        };

        Ok(ReadUTFChar { read_utf: read })
    }
}

impl Iterator for ReadUTFChar {
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
