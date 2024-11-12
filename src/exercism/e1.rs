fn reverse(input: &str) -> String {
    let reversed: String = input.chars().rev().collect();
    reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("abc"), "cba".to_string());
    }
}
