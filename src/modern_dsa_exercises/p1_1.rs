fn gcd(a:u32, b:u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(56, 98), 14);
        assert_eq!(gcd(20, 30), 10);
        assert_eq!(gcd(17, 13), 1);
    }
}
