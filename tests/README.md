# ReadUTF8 tests

## How to run tests? (for linux)

Firstly, you should go at the root of this repo and run
```sh
python3 tests_files/create_test_custom_delim_file_corrupt.py && python3 tests_files/create_test_custom_delim_file.py
```

Then, you can run:
```sh
cargo test
```

## Structure

- **src/utils/tests_utils.rs**: give utilities to test easily
    - **cmp_vector**: compare 2 vectors of string
    - **convert_string_to_list**: This library uses commands like **cat** to test, so we get a string output, and we must parse it to test.
    - **clean_str**: remove all extra �
        - Example: "My���� na��me is���" => "My� na�me is�"
        - Why? : because the numbers of � can be different, the numbers of � depends on how you deal with errors
        - This function should be removed in next version
    - **get_custom_delims**: A list of hard coded delimiters: "**::**", "**:;**", "**|**", "**éè**", "**小六号**", "**毫**"

- **tests_files/\*.rs**: all test files => reference, corrupted files...

- **tests/\*.rs**: all rust test files

## To know

It doesn't match the numbers of � that should be placed, it detects the same amount of errors, but it doesn't print the right numbers of � (it depends of the len of encoded char).