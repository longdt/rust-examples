fn i32_string_size(mut x: i32) -> usize {
    let mut d = 1;
    if x >= 0 {
        d = 0;
        x = -x;
    }
    let mut p = -10;
    for i in 1..10 {
        if x > p {
            return i + d;
        }
        p = 10 * p;
    }
    10 + d
}

#[inline]
pub fn i32_to_string_java(number: i32) -> String {
    let size = i32_string_size(number);
    let mut buf = vec![0u8; size];
    get_chars(number, size, &mut buf);
    unsafe { String::from_utf8_unchecked(buf) }
}

fn get_chars(mut i: i32, index: usize, buf: &mut [u8]) {
    let negative = i < 0;
    if !negative {
        i = -i;
    }

    let mut q;
    let mut r: usize;
    let mut char_pos = index;
    while i <= -100 {
        q = i / 100;
        r = ((q * 100) - i) as usize;
        i = q;
        char_pos -= 1;
        buf[char_pos] = DIGIT_ONES[r];
        char_pos -= 1;
        buf[char_pos] = DIGIT_TENS[r];
    }
    q = i / 10;
    r = ((q * 10) - i) as usize;
    char_pos -= 1;
    buf[char_pos] = b'0' + r as u8;

    if q < 0 {
        char_pos -= 1;
        buf[char_pos] = b'0' - q as u8;
    }
    if negative {
        char_pos -= 1;
        buf[char_pos] = b'-';
    }
}

const DIGIT_TENS: [u8; 100] = [
    b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
    b'1', b'1', b'1', b'1', b'1', b'1', b'1', b'1', b'1', b'1',
    b'2', b'2', b'2', b'2', b'2', b'2', b'2', b'2', b'2', b'2',
    b'3', b'3', b'3', b'3', b'3', b'3', b'3', b'3', b'3', b'3',
    b'4', b'4', b'4', b'4', b'4', b'4', b'4', b'4', b'4', b'4',
    b'5', b'5', b'5', b'5', b'5', b'5', b'5', b'5', b'5', b'5',
    b'6', b'6', b'6', b'6', b'6', b'6', b'6', b'6', b'6', b'6',
    b'7', b'7', b'7', b'7', b'7', b'7', b'7', b'7', b'7', b'7',
    b'8', b'8', b'8', b'8', b'8', b'8', b'8', b'8', b'8', b'8',
    b'9', b'9', b'9', b'9', b'9', b'9', b'9', b'9', b'9', b'9',
];

const DIGIT_ONES: [u8; 100] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_to_string() {
        let i = 10123;
        assert_eq!("10123", i32_to_string_java(i));
    }

    #[test]
    fn negative_to_string() {
        let i = -45610;
        assert_eq!("-45610", i32_to_string_java(i));
    }
}
