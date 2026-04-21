pub fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut double = false;

    for c in cc_number.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            if double {
                let double_digit = digit * 2;
                sum +=
                    if double_digit > 9 { double_digit - 9 } else { double_digit };
            } else {
                sum += digit;
            }
            double = !double;
        } else {
            continue;
        }
    }

    sum % 10 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }

    #[test]
    fn test_too_short() {
        assert!(!luhn("1"));
        assert!(luhn(""));
    }

    #[test]
    fn test_non_digit_chars() {
        assert!(!luhn("4263 9826 4026 929X"));
        assert!(!luhn("1234-5678-9012-3456"));
    }

    #[test]
    fn test_double_digit_calculation() {
        assert!(luhn("91")); // 9*2=18 -> 1+8=9; 9+1=10
        assert!(!luhn("92")); // 9*2=18 -> 1+8=9; 9+2=11
    }
}
