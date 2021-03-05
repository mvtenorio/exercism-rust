#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        _ if first_list == second_list => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (l1, l2) if l1 > l2 && is_superlist(first_list, second_list) => Comparison::Superlist,
        _ if is_superlist(second_list, first_list) => Comparison::Sublist,
        _ => Comparison::Unequal,
    }
}

fn is_superlist<T: PartialEq>(larger: &[T], smaller: &[T]) -> bool {
    larger.windows(smaller.len()).any(|w| w == smaller)
}
