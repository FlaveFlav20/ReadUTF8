# read_utf source code

## What's inside?

There are some utils like:

- **utils/non_ascii_char.rs**:
    - **check_non_ascii**: check if a char is ascii
    - **check_number_bytes_begin**: give the total size of encodes bytes by giving the first non-ascii character => see utf-8
    - **check_ascii_follow**: check if the byte is an ascii follow
- **tests_utils.rs**: just for test => take a look at **src/tests/README.md**

- **src/read_delims.rs**:
    - **ReadDelimiter**: a structure that will be used to read our file byte by byte
    - it contains an iterator to read the file