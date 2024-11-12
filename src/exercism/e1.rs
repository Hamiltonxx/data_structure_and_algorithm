fn reverse(input: &str) -> String {
    let reversed: String = input.chars().rev().collect();
    reversed
}

use chrono::{DateTime, Duration, Local};
fn gigasecond(start: DateTime<Local>) -> DateTime<Local> {
    start + Duration::seconds(1_000_000_000)
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("abc"), "cba".to_string());
    }

    #[test]
    fn test_gigasecond() {
        let now = Local::now();
        println!("gigaseconds after: {}", gigasecond(now));
    }
}
