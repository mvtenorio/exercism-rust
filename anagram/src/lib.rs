use std::collections::HashSet;

use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let word_graphemes = get_sorted_graphemes(&word);
    let word_hash = symmetric_fast_hash(&word);

    possible_anagrams
        .iter()
        .cloned()
        .filter(|w| {
            if word.len() != w.len() {
                return false;
            }

            let lw = w.to_lowercase();
            if word == lw || word_hash != symmetric_fast_hash(&lw) {
                return false;
            }

            word_graphemes == get_sorted_graphemes(&lw)
        })
        .collect()
}

fn get_sorted_graphemes(word: &str) -> Vec<&str> {
    let mut graphemes = word.graphemes(true).collect::<Vec<&str>>();
    graphemes.sort_unstable();

    graphemes
}

pub fn symmetric_fast_hash(word: &str) -> u8 {
    let mut hash: u8 = 0;

    for byte in word.as_bytes() {
        hash ^= byte;
    }

    hash
}
