use core::panic;

use crate::read_utf8::ReadUTF8;

pub struct ReadUTF8Delims {
    pub read_utf: ReadUTF8,
}

impl ReadUTF8Delims {
    ///
    /// For arguments => See ReadUTF8
    /// 
    pub fn new(
        path: String,
        delimiter: Vec<String>,
        print_invalid_char: Option<bool>,
        buffer_size: Option<usize>,
    ) -> Result<ReadUTF8Delims, std::io::Error> {
        let read = match ReadUTF8::new(path, Some(delimiter), print_invalid_char, buffer_size) {
            Ok(res) => res,
            _ => {
                panic!("Unable to create ReadUTF8Delims")
            }
        };

        Ok(ReadUTF8Delims { read_utf: read })
    }
}

impl Iterator for ReadUTF8Delims {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read_utf.read_delim() {
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
