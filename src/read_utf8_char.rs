//! # read_utf8_chars
//!
//! `read_utf8_chars` is a collection of utilities to read a file character by 
//! character

use core::panic;

use crate::read_utf8::ReadUTF8;

///
/// [ReadUTF8Char]: The structure to read all chaacters of a file. \
/// You can iterate throught this structure with 
/// my_ReadUTF8Char.into_iter()
/// 
pub struct ReadUTF8Char {
    #[doc = r" The [ReadUTF8] structure"]
    pub read_utf: ReadUTF8,
}

impl ReadUTF8Char {
    ///
    /// For arguments => See [ReadUTF8] \
    /// With this struct, you needn't to close the file if you iterate throught
    /// all elements 
    ///
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
