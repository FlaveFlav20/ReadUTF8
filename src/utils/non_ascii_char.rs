/// Deal with non-ascii chars

/// Ascii => 0b0[01]{7}
const CHECK_NON_ASCII: u8 = 0b10000000;
/// Non-ascii => 0b10[01]{6}
const SECOND_CHECK_NON_ASCII: u8 = 0b01000000;
/// Non-ascii wrote with 2 chars => 0b110[01]{5}
const CHECK_LEN_2: u8 = 0b11000000;
const SECOND_CHECK_LEN_2: u8 = 0b00100000;
/// Non-ascii wrote with 3 chars => 0b1110[01]{4}
const CHECK_LEN_3: u8 = 0b1110000;
const SECOND_CHECK_LEN_3: u8 = 0b00010000;
/// Non-ascii wrote with 4 chars => 0b11110[01]{3}
const CHECK_LEN_4: u8 = 0b1111000;
const SECOND_CHECK_LEN_4: u8 = 0b00001000;

/// All others non ascii chars that follow the first are 0b10[01]{6}
const CHECK_FOLLOW: u8 = 0b10000000;
const SECOND_CHECK_FOLLOW: u8 = 0b01000000;

pub fn check_non_ascii(c: u8) -> bool {
    (c & CHECK_NON_ASCII) != 0
}

pub fn check_number_bytes_begin(c: u8) -> usize {
    if (c & CHECK_NON_ASCII) != 0 && (!c & SECOND_CHECK_NON_ASCII) != 0 {
        return 1;
    } else if c & CHECK_LEN_2 != 0 && (!c & SECOND_CHECK_LEN_2) != 0 {
        return 2;
    } else if c & CHECK_LEN_3 != 0 && (!c & SECOND_CHECK_LEN_3) != 0 {
        return 3;
    } else if c & CHECK_LEN_4 != 0 && (!c & SECOND_CHECK_LEN_4) != 0 {
        return 4;
    }
    return 0;
}

pub fn check_ascii_follow(c: u8) -> bool {
    let _ha = c & CHECK_FOLLOW;
    let _b = !c & SECOND_CHECK_FOLLOW;
    let _c = !c;
    if c & CHECK_FOLLOW != 0 && (!c & SECOND_CHECK_FOLLOW) != 0 {
        return true;
    }
    return false;
}
