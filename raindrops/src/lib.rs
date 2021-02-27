pub fn raindrops(n: u32) -> String {
    let is_divisible_by = |factor| n % factor == 0;
    let mut result = String::new();

    if is_divisible_by(3) {
        result.push_str("Pling")
    }
    if is_divisible_by(5) {
        result.push_str("Plang")
    }
    if is_divisible_by(7) {
        result.push_str("Plong")
    }

    if result.is_empty() {
        n.to_string()
    } else {
        result
    }
}
