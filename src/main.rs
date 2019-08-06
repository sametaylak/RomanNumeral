trait RomanNumeral {
    const ROMAN_PAIRS: &'static[(&'static str, i32)] = &[
        ("M", 1000), ("CM", 900), ("D",  500), ("CD", 400),
        ("C", 100),  ("XC", 90),  ("L",  50),  ("XL", 40),
        ("X", 10),   ("IX", 9),   ("V",  5),   ("IV", 4),
        ("I", 1) ];

    fn to_roman_numeral(&self) -> String;
}

impl RomanNumeral for i32 {
    fn to_roman_numeral(&self) -> String {
        let mut out = String::new();
        let mut n = self.clone();
        for &(name, value) in Self::ROMAN_PAIRS.iter() {
            while n >= value {
                n -= value;
                out.push_str(name);
            }
        }
        assert!(n == 0);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_roman_numeral() {
        assert_eq!(1.to_roman_numeral(), "I");
        assert_eq!(2.to_roman_numeral(), "II");
        assert_eq!(3.to_roman_numeral(), "III");
        assert_eq!(4.to_roman_numeral(), "IV");
        assert_eq!(5.to_roman_numeral(), "V");
        assert_eq!(6.to_roman_numeral(), "VI");
        assert_eq!(7.to_roman_numeral(), "VII");
        assert_eq!(8.to_roman_numeral(), "VIII");
        assert_eq!(9.to_roman_numeral(), "IX");
        assert_eq!(10.to_roman_numeral(), "X");
        assert_eq!(15.to_roman_numeral(), "XV");
        assert_eq!(19.to_roman_numeral(), "XIX");
    }
}

fn main() {
    println!("{}", 8.to_roman_numeral());
}
