# read_utf

## Intro

This library allow you to read a file character by character/delims by delims with utf-8 compliant. \
It's also dealing with errors, by printing on stderr and placing a �

## Example

### ReadUTF8Delims

```rs
use crate::read_utf8_delims::ReadUTF8Delims;

fn main() {
    let path: &str = "my_path";
    let mut delims: Vec<String> = Vec::new();
    delims.push(String::from("\n"));
    let read: ReadUTF8Delims = ReadUTF8Delims::new(
            path.to_string(),
            delims,
            None,
            None,
        )
        .expect("Unable to init ReadUTF");

    let res: Vec<String> = read.into_iter().collect();
}
```

### ReadUTF8Char

```rs
use crate::read_utf8_char::ReadUTF8Char;

fn main() {
    let path: &str = "./tests_files/DDHC.txt";
    let read: ReadUTF8Char = ReadUTF8Char::new(
            path.to_string(),
            None,
            None,
        )
        .expect("Unable to init ReadUTF");

    let res: Vec<String> = read.into_iter().collect();
}
```

### ReadUTF8

Readl delim(s) by delim(s):
```rs
use crate::read_utf8::ReadUTF8;

fn main() {
    let path: &str = "./tests_files/DDHC.txt";
    let mut delims: Vec<String> = Vec::new();
    delims.push(String::from("\n"));
    let mut read: ReadUTF8 = ReadUTF8::new(
            path.to_string(),
            Some(delims),
            None,
            None,
        )
        .expect("Unable to init ReadUTF");

    let mut res: Vec<String> = Vec::new();

    while read.read_delim().expect("Unable to read") {
        res.push(read.line.to_string());
    }
    read.close();
}
```

Read char by char:
```rs
use crate::read_utf8::ReadUTF8;

fn main() {
    let path: &str = "./tests_files/DDHC.txt";
    let mut delims: Vec<String> = Vec::new();
    delims.push(String::from("\n"));
    let mut read: ReadUTF8 = ReadUTF8::new(
            path.to_string(),
            Some(delims),
            None,
            None,
        )
        .expect("Unable to init ReadUTF");

    let mut res: Vec<String> = Vec::new();

    while read.read_char().expect("Unable to read") {
        res.push(read.line.to_string());
    }
    read.close();
}
```

## Arguments

### ReadUTF8

```rs
...
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
    ...
    }
...
    /// 
    /// A method to get the line
    /// return:
    ///     - true if the line is valid
    ///     - false otherwise
    /// The line is located in "self.line" attribute
    /// 
    pub fn read_delim(&mut self) -> Result<bool, std::io::Error> {
        ...
    }
...
    /// 
    /// A method to get the line
    /// return:
    ///     - true if the line is valid
    ///     - false otherwise
    /// The character is located in "self.line" attribute
    /// 
    pub fn read_char(&mut self) -> Result<bool, std::io::Error> {
        ...
    }
...
}

```

### ReadUTF8Delims

```rs
...
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
        ...
    }
...
```

### ReadUTF8Char

```rs
...
impl ReadUTF8Char {
    ///
    /// For arguments => See ReadUTF8
    /// 
    pub fn new(
        path: String,
        print_invalid_char: Option<bool>,
        buffer_size: Option<usize>,
    ) -> Result<ReadUTF8Char, std::io::Error> {
        ...
    }
...
```

## Structure

- **src/**: source code
- **tests/**: rust tests
- **tests_files/**: files to test





















