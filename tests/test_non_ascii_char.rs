use read_delims::utils::non_ascii_char;

mod tests_non_ascii_char {
    use super::*;

    #[test]
    fn test_ascii_char() {
        for c in 0..128 {
            assert_eq!(
                non_ascii_char::check_non_ascii(c as u8),
                false,
                "The character {} is detected as non ascii char...",
                c as u8 as char
            );
        }
    }

    #[test]
    fn test_non_ascii_char() {
        for c in 128..255 {
            assert_eq!(
                non_ascii_char::check_non_ascii(c as u8),
                true,
                "The character {} is detected as ascii char...",
                c as u8 as char
            );
        }
    }
}
