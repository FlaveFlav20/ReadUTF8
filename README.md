# read_utf

## Intro

This library allow you to read a file character by character/delims by delims with utf-8 compliant. \
It's also dealing with errors, by printing on stderr and placing a �

## Example

### ReadUTFDelims

```rs
use read_utf::read_utf_delims::ReadUTFDelims;

fn main() {
    let delims: Vec<String> = Vec::from(String::from("\n"))
    let read: ReadUTFDelims = ReadUTFDelims::new(
            PATH_CUSTOM_DELIM_ERROR.to_string(),
            delims,
            1024,
        )
        .expect("Unable to init ReadUTF");

    let res: Vec<String> = read.into_iter().collect();
}
```

### ReadUTFChar

```rs
use read_utf::read_utf_char::ReadUTFChar;

fn main() {
    let delims: Vec<String> = Vec::from(String::from("\n"))
    let read: ReadUTFChar = ReadUTFChar::new(
            PATH_CUSTOM_DELIM_ERROR.to_string(),
            delims,
            1024,
        )
        .expect("Unable to init ReadUTF");

    let res: Vec<String> = read.into_iter().collect();
}
```

## Structure

- **src/**: source code
- **tests/**: rust tests
- **tests_files/**: files to test