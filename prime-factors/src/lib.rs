use std::iter;

pub fn factors(n: u64) -> Vec<u64> {
    let mut result = vec![];
    let mut candidates = iter::once(2).chain((3..n).step_by(2));
    let mut rem = n;

    while rem > 1 {
        let p = candidates.next().unwrap();
        while rem % p == 0 {
            rem = rem / p;
            result.push(p);
        }
    }

    result
}
