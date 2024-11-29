use read_delims::read_delims::ReadDelimiter;
use read_delims::utils::tests_utils::{cmp_vector, convert_string_to_list, get_custom_delims};
use std::process::Command;

static PATH: &str = "./tests_files/DDHC.txt";
static PATH_CUSTOM_DELIM: &str = "./tests_files/DDHC_custom_delims.txt";

static PATH_CUSTOM_DELIM_ERROR_REF: &str = "./tests_files/DDHC_custom_delims_byte_ref.txt";
static PATH_CUSTOM_DELIM_ERROR: &str = "./tests_files/DDHC_custom_delims_byte.txt";

mod tests_read_custom_delim {

    use super::*;

    #[test]
    fn simple_read() {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("cat '".to_string() + PATH)
                .output()
                .expect("failed to execute process")
        };

        let ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };

        let ref_: Vec<String> = convert_string_to_list(ref_str);

        let mut read: ReadDelimiter =
            ReadDelimiter::new(PATH_CUSTOM_DELIM.to_string(), get_custom_delims(), 1024)
                .expect("Unable to init ReadDelimiter");

        let mut res: Vec<String> = Vec::new();
        while read.read().expect("Unable to read delimiter") != true {
            res.push(read.line.to_string());
        }
        cmp_vector(res, ref_);
    }

    #[test]
    fn simple_read_with_errors() {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "Not available on windows"])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("cat '".to_string() + PATH_CUSTOM_DELIM_ERROR_REF)
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
        while read.read().expect("Unable to read delimiter") != true && index < limit {
            index += 1;
            res.push(read.line.to_string());
        }
        assert_eq!(index, limit);
        cmp_vector(res, ref_);
    }
}
