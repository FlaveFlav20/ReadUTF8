use read_delims::read_delims::ReadDelimiter;
use read_delims::utils::tests_utils::{cmp_vector, convert_string_to_list};
use std::process::Command;

static PATH: &str = "./tests_files/DDHC.txt";

mod tests_read_eol {

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
        let mut delim: Vec<String> = Vec::new();
        delim.push(String::from("\n"));
        let mut read: ReadDelimiter = ReadDelimiter::new(PATH.to_string(), delim, 1024)
            .expect("Unable to init ReadDelimiter");

        let mut res: Vec<String> = Vec::new();
        while read.read().expect("Unable to read delimiter") != true {
            res.push(read.line.to_string());
        }
        cmp_vector(res, ref_);
    }
}
