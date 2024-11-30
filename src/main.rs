pub mod read_utf8;
pub mod read_utf8_char;
pub mod read_utf8_delims;
pub mod utils;

use crate::read_utf8::ReadUTF8;

fn main() {
    let path: &str = "./tests_files/DDHC.txt";
    let mut delims: Vec<String> = Vec::new();
    delims.push(String::from("\n"));
    let mut read: ReadUTF8 =
        ReadUTF8::new(path.to_string(), Some(delims), None, None).expect("Unable to init ReadUTF");

    let mut res: Vec<String> = Vec::new();

    while read.read_char().expect("Unable to read") {
        res.push(read.line.to_string());
    }
    read.close();
}
