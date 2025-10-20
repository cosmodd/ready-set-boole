pub fn gray_code(n: u32) -> u32 {
    let mut result = n & 0x80000000;
    let mut previous_bit: u32 = result;

    for i in (0u32..31).rev() {
        let bit = (n >> i) & 1;
        result |= (bit ^ previous_bit) << i;
        previous_bit = bit;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::gray_code::gray_code;

    #[test]
    fn test_gray_code() {
        assert_eq!(gray_code(0), 0);
        assert_eq!(gray_code(1), 1);
        assert_eq!(gray_code(2), 3);
        assert_eq!(gray_code(3), 2);
        assert_eq!(gray_code(4), 6);
        assert_eq!(gray_code(5), 7);
        assert_eq!(gray_code(6), 5);
        assert_eq!(gray_code(7), 4);
        assert_eq!(gray_code(8), 12);
    }
}