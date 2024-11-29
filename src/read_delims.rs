use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::mem::ManuallyDrop;

use crate::utils::non_ascii_char;
pub struct ReadDelimiter {
    pub _filename: String,
    file: ManuallyDrop<File>,
    file_drop: bool,
    pub delimiter: Vec<String>,
    pub line: String,
    buffer: Vec<u8>,
    // save_buffer is used if there is an error while parsing non-ascii chars
    //          -> the size should be 0 or 1, it an error otherwise
    save_buffer: VecDeque<u8>,
    index_buffer: usize,
    curr_index: usize,
}
/*
    ReadDelimiter:
        - Goal: Create a structure to read a file delim by delim (like line by line)
*/
impl ReadDelimiter {
    pub fn new(
        path: String,
        delimiter: Vec<String>,
        buffer_size: usize,
    ) -> Result<ReadDelimiter, std::io::Error> {
        let file = File::open(&path)?;
        Ok(ReadDelimiter {
            _filename: path.clone(),
            file: ManuallyDrop::new(file),
            file_drop: false,
            delimiter: delimiter.clone(),
            line: "".to_string(),
            buffer: vec![0; buffer_size],
            save_buffer: VecDeque::new(),
            index_buffer: 0,
            curr_index: 0,
        })
    }

    /// Read all bytes from the delims, and return the numbers of bytes read
    pub fn read(&mut self, print_invalid_char: bool) -> Result<bool, std::io::Error> {
        if self.file_drop {
            if print_invalid_char {
                eprintln!("Unable to read the file, because it is closed");
            }
            return Ok(false);
        }
        self.line = "".to_string();
        let mut buffer: u8 = 0;

        while let Ok(bytes_read) = read_from_buffer(self, &mut buffer, print_invalid_char) {
            if bytes_read == 0 {
                break;
            }

            if non_ascii_char::check_non_ascii(buffer) {
                let _ = read_non_ascii_char(self, buffer, print_invalid_char);
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

    pub fn close(&mut self) {
        if !self.file_drop {
            return;
        }
        let file = unsafe { ManuallyDrop::take(&mut self.file) };
        drop(file);
        self.file_drop = !self.file_drop;
    }
}

fn read_from_buffer(
    read_delim: &mut ReadDelimiter,
    c: &mut u8,
    print_invalid_char: bool,
) -> Result<usize, std::io::Error> {
    if read_delim.save_buffer.len() != 0 {
        match read_delim.save_buffer.pop_front() {
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
    } else if read_delim.curr_index >= read_delim.index_buffer {
        let bytes_read = match (read_delim.file).read(&mut read_delim.buffer) {
            Ok(bytes_read) => bytes_read,
            Err(_e) => panic!("[ReadDeliiter][read_from_buffer]: Error while reading file"),
        };

        if bytes_read == 0 {
            return Ok(0);
        }

        read_delim.curr_index = 0;
        read_delim.index_buffer = bytes_read;
    }
    *c = read_delim.buffer[read_delim.curr_index] as u8;
    read_delim.curr_index += 1;
    return Ok(1 as usize);
}

fn read_non_ascii_char(
    read_delim: &mut ReadDelimiter,
    first_u8: u8,
    print_invalid_char: bool,
) -> Result<(), std::io::Error> {
    let size: usize = non_ascii_char::check_number_bytes_begin(first_u8);

    if size <= 0 && print_invalid_char {
        read_delim.line.push('�');
        eprintln!("Not a valid character!");
        return Ok(());
    }

    let mut chars: Vec<u8> = Vec::new();
    chars.push(first_u8);
    let mut buffer: u8 = 0;
    for _ in 1..size {
        let bytes_read = match read_from_buffer(read_delim, &mut buffer, print_invalid_char) {
            Ok(bytes_read) => bytes_read,
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "[ReadDelimiter][read_non_ascii_char]: Error reading file: {}",
                        e
                    ),
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
        read_delim.line.push_str(valid_str);
    } else {
        for i in 1..size {
            read_delim.save_buffer.push_back(chars[i]);
        }
        read_delim.line.push('�');
        if print_invalid_char {
            eprintln!("Unable to get char?");
        }
    }
    Ok(())
}
