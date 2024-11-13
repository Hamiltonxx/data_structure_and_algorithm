use std::collections::HashSet;

fn sort(input: &str) -> Vec<char> {
    let mut chars: Vec<char> = input.chars().collect();
    chars.sort();
    chars
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // check if the two words have the same characters in the same frequency
    let lc_word = word.to_lowercase();
    let sorted = sort(&lc_word);
    possible_anagrams.iter().filter_map(|&poss| {
        let lc_poss = poss.to_lowercase();
        if sort(&lc_poss) == sorted && lc_poss != lc_word {
            Some(poss)
        } else {
            None
        }
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_matches() {
        let word = "diaper";
        let inputs = &["hello", "world", "zombies", "pants"];
        let output = anagrams_for(word, inputs);
        let expected = HashSet::from([]);
        assert_eq!(output, expected);
    }

    #[test]
    fn detects_two_anagrams() {
        let output = anagrams_for("solemn", &["lemons", "cherry", "melons"]);
        let expected = HashSet::from(["lemons", "melons"]);
        assert_eq!(output, expected);
    }
}
