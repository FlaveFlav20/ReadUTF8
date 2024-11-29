/// Only for tests folder

pub fn cmp_vector(vec1: Vec<String>, vec2: Vec<String>) -> () {
    assert_eq!(
        vec1.len(),
        vec2.len(),
        "Not the same len, vec1.len() (ref): \"{}\"; vec2.len() (to test): \"{}\"",
        vec1.len(),
        vec2.len()
    );

    for i in 0..vec1.len() {
        assert_eq!(
            clean_str(vec1[i].to_string()),
            clean_str(vec2[i].to_string()),
            "Not the same! i: {}; vec1[i] (ref): \"{}\"; vec2[i] (to test): \"{}\"",
            i,
            vec1[i],
            vec2[i]
        );
    }
}

pub fn convert_string_to_list(str: String) -> Vec<String> {
    let mut convert: Vec<String> = str.split('\n').map(|e| e.to_string()).collect();
    if convert.len() == 1 && convert[0] == "".to_string() {
        convert = Vec::new();
    }

    /*
        if we have "blablabla\n" the tail command return ["blablabla", ""], so we must remove it because of sed command
    */

    if convert.len() > 1 && convert[convert.len() - 1].is_empty() {
        convert.remove(convert.len() - 1);
    }
    convert
}

fn clean_str(str: String) -> String {
    let mut res: String = String::from("");

    let mut i: usize = 0;

    while i < str.len() {
        if str.as_bytes()[i] == '�' as u8 {
            res += &String::from(str.as_bytes()[i] as char);
            while str.as_bytes()[i] == '�' as u8 && i < str.len() {
                i += 1;
            }
        }
        i += 1;
    }

    res
}

pub fn get_custom_delims() -> Vec<String> {
    let mut delims: Vec<String> = Vec::new();
    delims.push(String::from("::"));
    delims.push(String::from(":;"));
    delims.push(String::from("|"));
    delims.push(String::from("éè"));
    delims.push(String::from("小六号"));
    delims.push(String::from("毫"));
    delims
}
