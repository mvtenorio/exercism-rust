pub fn build_proverb(list: &[&str]) -> String {
    list.windows(2)
        .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
        .chain(
            list.iter()
                .take(1)
                .map(|i| format!("And all for the want of a {}.", i)),
        )
        .collect()
}
