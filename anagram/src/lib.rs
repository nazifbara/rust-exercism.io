use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    let word_lower = lower_string(word);
    let word_chars_sorted = sort_chars(&word_lower);

    for anagram in possible_anagrams {
        let anagram_lower = lower_string(anagram);

        if word_lower == anagram_lower {
            continue;
        }

        let anagram_chars_sorted = sort_chars(&anagram_lower);

        if word_chars_sorted == anagram_chars_sorted {
            result.insert(*anagram);
        }
    }
    result
}

fn lower_string(word: &str) -> String {
    word.chars()
        .map(|c| c.to_lowercase().next().unwrap())
        .collect::<String>()
}

fn sort_chars(word: &String) -> Vec<char> {
    let mut word_chars = word.chars().collect::<Vec<char>>();
    word_chars.sort_unstable();
    word_chars
}
