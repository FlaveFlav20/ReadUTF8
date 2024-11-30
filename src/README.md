# ReadUTF8 source code

## What's inside?

There are some utils like:

- **utils/non_ascii_char.rs**:
    - **check_non_ascii**: check if a char is ASCII
    - **check_number_bytes_begin**: give the total size of encodes bytes by giving the first non-ASCII character => see utf-8
    - **check_ascii_follow**: check if the byte is an ASCII follow

- **utils/tests_utils.rs**: just for test => take a look at **src/tests/README.md**

- **read_utf8.rs**:
    - **struct ReadUTF8**: it will be used to read our file byte by byte or delim(s) by delim(s)
    - It is the main structure, the others structures **ReadUTF8Delims**/**ReadUTF8&Char** is just calling it

- **read_utf8_delims.rs**
    - **struct ReadUTFDelims**: a structure to have a kind interface to read delim by delim

- **read_utf8_char.rs**:
    - **struct ReadUTFChar**: a structure to have a kind interface to read char by char