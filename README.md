# ReadUTF

## Intro

This library allow you to read a file character by character/delims by delims with utf-8 compliant. \
It's also dealing with errors, by printing on stderr and placing a �

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

    let res: Vec<String> = read.into_iter().collect();

    
}
```

## Structure

- **src/**: source code
- **tests/**: rust tests
- **tests_files/**: files to test


## To know

It doesn't match the numbers of � that should be placed, it detect the same amount of errors, but it doesn't print the right numbers of � (it depends of the len of encoded char).