use read_utf8::read_utf8_char::ReadUTF8Char;
use std::process::Command;

static PATH: &str = "./tests_files/DDHC.txt";

mod tests_read_char {

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
                .arg("cat ".to_string() + PATH)
                .output()
                .expect("failed to execute process")
        };

        let ref_str: String = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(_e) => panic!("Error convertion"),
        };
        let char_vec: Vec<char> = ref_str.chars().collect();

        let read: ReadUTF8Char =
            ReadUTF8Char::new(PATH.to_string(), None, None).expect("Unable to read ReadUTFChar");
        let mut len: usize = 0;

        for c in read.into_iter() {
            len += 1;
            assert_eq!(String::from(char_vec[len - 1]), c);
        }

        assert_eq!(char_vec.len(), len);
    }
}
