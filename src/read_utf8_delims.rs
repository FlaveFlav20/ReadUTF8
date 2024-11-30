//! # read_utf8_chars
//!
//! `read_utf8_chars` is a collection of utilities to read a file delimiter 
//! by delimiter



use core::panic;

use crate::read_utf8::ReadUTF8;

///
/// [ReadUTF8Char]: The structure to read all file delimiters by delimiters. \
/// You can iterate throught this structure with 
/// my_ReadUTF8Delims.into_iter()
///
pub struct ReadUTF8Delims {
    #[doc = r" The [ReadUTF8] structure"]
    pub read_utf: ReadUTF8,
}

impl ReadUTF8Delims {
    ///
    /// For arguments => See [ReadUTF8] \
    /// You can iterate throught this structure with my_ReadUTF8Delims.into_iter()
    /// With this struct, you needn't to close the file if you iterate throught
    /// all elements 
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
