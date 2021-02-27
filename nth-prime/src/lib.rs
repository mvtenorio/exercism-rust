pub fn nth(n: u32) -> u32 {
    match n {
        0 => 2,
        _ => (3..)
            .step_by(2)
            .filter(|x| is_prime(*x))
            .nth(n as usize - 1)
            .unwrap(),
    }
}

fn is_prime(x: u32) -> bool {
    !(2..=x / 2).any(|i| x % i == 0)
}
