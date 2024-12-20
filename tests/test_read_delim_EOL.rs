use read_utf8::read_utf8_delims::ReadUTF8Delims;
use read_utf8::utils::tests_utils::{cmp_vector, convert_string_to_list};
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
            Command::new("bash")
                .arg("-c")
                .arg("cat ".to_string() + PATH)
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
        let read: ReadUTF8Delims = ReadUTF8Delims::new(PATH.to_string(), delim, None, None)
            .expect("Unable to init ReadUTF");
        let res: Vec<String> = read.into_iter().collect();
        cmp_vector(ref_, res);
    }
}
