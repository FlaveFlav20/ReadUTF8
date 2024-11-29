pub mod read_delims;
pub mod utils;

use std::process::Command;

use crate::read_delims::ReadDelimiter;
use crate::utils::tests_utils::{cmp_vector, convert_string_to_list, get_custom_delims};

static PATH_CUSTOM_DELIM_ERROR_REF: &str = "./tests_files/DDHC_custom_delims_corrupted_ref.txt";
static PATH_CUSTOM_DELIM_ERROR: &str = "./tests_files/DDHC_custom_delims_corrupted.txt";

fn simple_read_with_errors() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "Not available on windows"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("cat ".to_string() + PATH_CUSTOM_DELIM_ERROR_REF)
            .output()
            .expect("failed to execute process")
    };

    let ref_str: String = match String::from_utf8(output.stdout) {
        Ok(string) => string,
        Err(_e) => panic!("Error convertion"),
    };

    let ref_: Vec<String> = convert_string_to_list(ref_str);

    let mut read: ReadDelimiter =
        ReadDelimiter::new(PATH_CUSTOM_DELIM_ERROR.to_string(), get_custom_delims(), 1024)
            .expect("Unable to init ReadDelimiter");

    let mut res: Vec<String> = Vec::new();
    let limit: usize = 1000;
    let mut index: usize = 0;
    while read.read().expect("Unable to read delimiter") == true && index < limit {
        index += 1;
        res.push(read.line.to_string());
    }

    for i in 0..res.len() {
        println!("1.:{}$", res[i]);
        println!("Bahahahahaha {}", i);
        println!("2.:{}$", ref_[i]);
    }

    assert_ne!(index, limit);
    cmp_vector(res, ref_);
}

fn main() {

    simple_read_with_errors();
}
