use crate::adder::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut result: u32 = 0;

    if b & 1 == 1 {
        result = a;
    }

    for i in 1u32..32 {
        if (b >> i) & 1 == 1 {
            result = adder(result, a << i)
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplier_with_loop() {
        for i in 0u32..100 {
            for j in 0u32..100 {
                assert_eq!(multiplier(i, j), i * j);
            }
        }
    }
}