use std::collections::HashSet;

use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word = &word.to_lowercase();

    possible_anagrams
        .iter()
        .cloned()
        .filter(|w| is_anagram(word, &w.to_lowercase()))
        .collect()
}

fn is_anagram(word: &str, possible_anagram: &str) -> bool {
    if word == possible_anagram {
        return false;
    }

    get_sorted_graphemes(word) == get_sorted_graphemes(possible_anagram)
}

fn get_sorted_graphemes(word: &str) -> Vec<&str> {
    let mut graphemes = word.graphemes(true).collect::<Vec<&str>>();
    graphemes.sort_unstable();

    graphemes
}
