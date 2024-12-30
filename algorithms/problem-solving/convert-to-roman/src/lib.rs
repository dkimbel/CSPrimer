struct Numeral {
    main_char: char,
    subtractive_char: Option<char>,
    value: u16,
}

const NUMERALS_BY_VALUE_DECREASING: [Numeral; 13] = [
    Numeral { main_char: 'M', subtractive_char: None, value: 1000 },
    Numeral { main_char: 'M', subtractive_char: Some('C'), value: 900 },
    Numeral { main_char: 'D', subtractive_char: None, value: 500 },
    Numeral { main_char: 'D', subtractive_char: Some('C'), value: 400 },
    Numeral { main_char: 'C', subtractive_char: None, value: 100 },
    Numeral { main_char: 'C', subtractive_char: Some('X'), value: 90 },
    Numeral { main_char: 'L', subtractive_char: None, value: 50 },
    Numeral { main_char: 'L', subtractive_char: Some('X'), value: 40 },
    Numeral { main_char: 'X', subtractive_char: None, value: 10 },
    Numeral { main_char: 'X', subtractive_char: Some('I'), value: 9 },
    Numeral { main_char: 'V', subtractive_char: None, value: 5 },
    Numeral { main_char: 'V', subtractive_char: Some('I'), value: 4 },
    Numeral { main_char: 'I', subtractive_char: None, value: 1 },
];

pub fn convert_to_roman(n: u16) -> String {
    let mut remaining_n = n;
    let mut result: String = String::new();

    for numeral in NUMERALS_BY_VALUE_DECREASING {
        while remaining_n >= numeral.value {
            if let Some(subtractive_char) = numeral.subtractive_char {
                result.push(subtractive_char);
            }
            result.push(numeral.main_char);
            remaining_n -= numeral.value;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let result = convert_to_roman(0);
        assert_eq!(result, "");
    }

    #[test]
    fn test_1() {
        let result = convert_to_roman(1);
        assert_eq!(result, "I");
    }

    #[test]
    fn test_13() {
        let result = convert_to_roman(13);
        assert_eq!(result, "XIII");
    }

    #[test]
    fn test_44() {
        let result = convert_to_roman(44);
        assert_eq!(result, "XLIV");
    }

    #[test]
    fn test_89() {
        let result = convert_to_roman(89);
        assert_eq!(result, "LXXXIX");
    }

    #[test]
    fn test_400() {
        let result = convert_to_roman(400);
        assert_eq!(result, "CD");
    }

    #[test]
    fn test_800() {
        let result = convert_to_roman(800);
        assert_eq!(result, "DCCC");
    }

    #[test]
    fn test_3997() {
        let result = convert_to_roman(3997);
        assert_eq!(result, "MMMCMXCVII");
    }
}
