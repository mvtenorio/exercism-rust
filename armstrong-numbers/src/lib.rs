pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.to_string();
    let len = digits.len() as u32;
    const RADIX: u32 = 10;

    digits
        .chars()
        .map(|c| c.to_digit(RADIX).unwrap())
        .map(|d| d.pow(len))
        .sum::<u32>()
        == num
}
