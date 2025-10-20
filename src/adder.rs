pub fn adder(a: u32, b: u32) -> u32 {
    let mut result: u32 = 0;
    let mut carry: u32 = 0;

    for i in 0u32..32 {
        let a_bit: u32 = (a >> i) & 1;
        let b_bit: u32 = (b >> i) & 1;

        let res_bit = a_bit ^ b_bit ^ carry;
        result |= res_bit << i;
        carry = a_bit & b_bit | ((a_bit | b_bit) & carry);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adder_loop() {
        for i in 0u32..1000 {
            for j in 0u32..1000 {
                assert_eq!(adder(i, j), i + j);
            }
        }
    }
}
