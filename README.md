# ReadUTF

## Intro

This library allow you to read a file character by character/delims by delims with utf-8 compliant. \
It's also dealing with errors, by printing on stderr and placing a �

## What's inside?

There are some utils like:
- **src/utils/non_ascii_char.rs**:
    - **check_non_ascii**: check if a char is ascii
    - **check_number_bytes_begin**: give the total size of encodes bytes by giving the first non-ascii character => see utf-8
    - **check_ascii_follow**: check if the byte is an ascii follow

- **src/read_delims.rs**:
    - **ReadDelimiter**: a structure that will be used to read our file byte by byte
    - **read(&mut self, print_invalid_char: bool)**: to read byte by byte.

## Example

### read_delims

```rs
use read_utf::ReadUTF;

fn main() {
    let delims: Vec<String> = Vec::from(String::from("\n"))
    let mut read: ReadUTF = ReadUTF::new(
            PATH_CUSTOM_DELIM_ERROR.to_string(),
            delims,
            1024,
        )
        .expect("Unable to init ReadUTF");

    let mut res: Vec<String> = Vec::new();
        let limit: usize = 1000;
        let mut index: usize = 0;
        while read.read_delim(false).expect("Unable to read delimiter") == true && index < limit {
            index += 1;
            res.push(read.line.to_string());
        }
}
```

## To know

It doesn't match the numbers of � that should be placed, it detect the same amount of errors, but it doesn't print the right numbers of � (it depends of the len of encoded char).