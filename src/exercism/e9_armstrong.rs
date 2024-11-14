// pub fn is_armstrong_number(num: u32) -> bool {
//     let s = num.to_string();
//     let exp = s.len() as u32;
//
//     let sum = s.chars().into_iter().fold(0, |acc,c| acc+c.to_digit(10).unwrap().pow(exp));
//     sum == num
// }

fn is_armstrong_number(num: u32) -> bool {
    let s = num.to_string();
    s.chars().fold(0, |acc,c| acc+c.to_digit(10).unwrap().pow(s.len() as u32)) == num
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_an_armstrong_number() {
        assert!(is_armstrong_number(0))
    }
    #[test]
    #[ignore]
    fn single_digit_numbers_are_armstrong_numbers() {
        assert!(is_armstrong_number(5))
    }
    #[test]
    #[ignore]
    fn there_are_no_two_digit_armstrong_numbers() {
        assert!(!is_armstrong_number(10))
    }
    #[test]
    #[ignore]
    fn three_digit_number_that_is_an_armstrong_number() {
        assert!(is_armstrong_number(153))
    }
    #[test]
    #[ignore]
    fn three_digit_number_that_is_not_an_armstrong_number() {
        assert!(!is_armstrong_number(100))
    }
    #[test]
    #[ignore]
    fn four_digit_number_that_is_an_armstrong_number() {
        assert!(is_armstrong_number(9_474))
    }
    #[test]
    #[ignore]
    fn four_digit_number_that_is_not_an_armstrong_number() {
        assert!(!is_armstrong_number(9_475))
    }
    #[test]
    #[ignore]
    fn seven_digit_number_that_is_an_armstrong_number() {
        assert!(is_armstrong_number(9_926_315))
    }
    #[test]
    #[ignore]
    fn seven_digit_number_that_is_not_an_armstrong_number() {
        assert!(!is_armstrong_number(9_926_314))
    }
}
