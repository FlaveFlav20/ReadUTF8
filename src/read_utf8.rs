use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::mem::ManuallyDrop;

use crate::utils::non_ascii_char;
pub struct ReadUTF8 {
    pub _filename: String,
    file: ManuallyDrop<File>,
    file_drop: bool,
    pub delimiter: Vec<String>,
    pub line: String,
    buffer: Vec<u8>,
    // save_buffer is used if there is an error while parsing non-ascii chars
    save_buffer: VecDeque<u8>,
    index_buffer: usize,
    curr_index: usize,
    print_invalid_char: bool,
}
/*
    ReadUTF8:
        - Goal: Create a structure to read a file delim by delim (like line by line)
*/

impl ReadUTF8 {
    ///
    /// path => mandatory
    /// delimiter/print_invalid_char/buffer_size are optionnal => you must pass
    ///  the argument with Some(your arg) or you can put None
    ///
    /// path => path to the file to read
    /// delimiter => a list of String that are delimiters
    /// print_invalid_char => if true, it prints an error when invalid char on
    ///                         stderr
    /// buffer_size => by default, it's 1024.
    ///             => The maximum buffer size when reading file, for example,
    ///                 when reading char by char, it will not read char by
    ///                 char, it will read an entire buffer, and give the char
    ///                 from the buffer
    ///
    pub fn new(
        path: String,
        delimiter: Option<Vec<String>>,
        print_invalid_char: Option<bool>,
        buffer_size: Option<usize>,
    ) -> Result<ReadUTF8, std::io::Error> {
        Ok(ReadUTF8 {
            _filename: path.clone(),
            file: ManuallyDrop::new(File::open(&path)?),
            file_drop: false,
            delimiter: match delimiter {
                None => Vec::new(),
                Some(delim) => delim,
            },
            line: "".to_string(),
            buffer: vec![
                0;
                match buffer_size {
                    None => 1024,
                    Some(size) => size,
                }
            ],
            save_buffer: VecDeque::new(),
            index_buffer: 0,
            curr_index: 0,
            print_invalid_char: match print_invalid_char {
                None => false,
                Some(b) => b,
            },
        })
    }
}

impl ReadUTF8 {
    ///
    /// A method to get the line
    /// return:
    ///     - true if the line is valid
    ///     - false otherwise
    /// The line is located in "self.line" attribute
    ///
    pub fn read_delim(&mut self) -> Result<bool, std::io::Error> {
        if self.file_drop {
            if self.print_invalid_char {
                eprintln!("Unable to read the file, because it is closed");
            }
            return Ok(false);
        }
        self.line = "".to_string();
        let mut buffer: u8 = 0;

        while let Ok(bytes_read) = self.read_from_buffer(&mut buffer, self.print_invalid_char) {
            if bytes_read == 0 {
                break;
            }

            if non_ascii_char::check_non_ascii(buffer) {
                let _ = self.read_non_ascii_char(buffer, self.print_invalid_char);
            } else {
                self.line += &(buffer as char).to_string();
            }

            for i in 0..self.delimiter.len() {
                if self.delimiter[i].as_bytes().len() == 0 {
                    continue;
                }
                if self.line.len() < self.delimiter[i].as_bytes().len() {
                    continue;
                }

                let str = self
                    .line
                    .get(self.line.len() - self.delimiter[i].len()..)
                    .unwrap_or("");

                if self.delimiter[i] == str {
                    for _i in 0..self.delimiter[i].chars().count() {
                        self.line.pop();
                    }
                    return Ok(true);
                }
            }
        }
        Ok(self.line.len() != 0)
    }

    ///
    /// A method to get the line
    /// return:
    ///     - true if the line is valid
    ///     - false otherwise
    /// The character is located in "self.line" attribute
    ///
    pub fn read_char(&mut self) -> Result<bool, std::io::Error> {
        if self.file_drop {
            if self.print_invalid_char {
                eprintln!("Unable to read the file, because it is closed");
            }
            return Ok(false);
        }
        self.line = "".to_string();
        let mut buffer: u8 = 0;

        match self.read_from_buffer(&mut buffer, self.print_invalid_char) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    return Ok(false);
                }

                if non_ascii_char::check_non_ascii(buffer) {
                    let _ = self.read_non_ascii_char(buffer, self.print_invalid_char);
                } else {
                    self.line += &(buffer as char).to_string();
                }
            }
            _ => return Ok(false),
        };

        Ok(self.line.len() != 0)
    }

    pub fn close(&mut self) {
        if !self.file_drop {
            return;
        }
        let file = unsafe { ManuallyDrop::take(&mut self.file) };
        drop(file);
        self.file_drop = !self.file_drop;
    }

    fn read_from_buffer(
        &mut self,
        c: &mut u8,
        print_invalid_char: bool,
    ) -> Result<usize, std::io::Error> {
        if self.save_buffer.len() != 0 {
            match self.save_buffer.pop_front() {
                Some(value) => {
                    *c = value;
                    return Ok(1 as usize);
                }
                _ => {
                    *c = 0;
                    if print_invalid_char {
                        eprint!("Error when read buffer");
                    }
                    return Ok(0 as usize);
                }
            }
        } else if self.curr_index >= self.index_buffer {
            let bytes_read = match (self.file).read(&mut self.buffer) {
                Ok(bytes_read) => bytes_read,
                Err(_e) => panic!("[ReadDeliiter][read_from_buffer]: Error while reading file"),
            };

            if bytes_read == 0 {
                return Ok(0);
            }

            self.curr_index = 0;
            self.index_buffer = bytes_read;
        }
        *c = self.buffer[self.curr_index] as u8;
        self.curr_index += 1;
        return Ok(1 as usize);
    }

    fn read_non_ascii_char(
        &mut self,
        first_u8: u8,
        print_invalid_char: bool,
    ) -> Result<(), std::io::Error> {
        let size: usize = non_ascii_char::check_number_bytes_begin(first_u8);

        if size <= 0 && print_invalid_char {
            self.line.push('�');
            eprintln!("Not a valid character!");
            return Ok(());
        }

        let mut chars: Vec<u8> = Vec::new();
        chars.push(first_u8);
        let mut buffer: u8 = 0;
        for _ in 1..size {
            let bytes_read = match self.read_from_buffer(&mut buffer, print_invalid_char) {
                Ok(bytes_read) => bytes_read,
                Err(e) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("[ReadUTF8][read_non_ascii_char]: Error reading file: {}", e),
                    ));
                }
            };

            if bytes_read == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "Unexpected EOF while reading multi-byte character",
                ));
            }
            chars.push(buffer);
        }

        if let Ok(valid_str) = std::str::from_utf8(&chars) {
            self.line.push_str(valid_str);
        } else {
            for i in 1..size {
                self.save_buffer.push_back(chars[i]);
            }
            self.line.push('�');
            if print_invalid_char {
                eprintln!("Unable to get char?");
            }
        }
        Ok(())
    }
}
