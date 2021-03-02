#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }

    for i in 0..first_list.len() {
        if first_list[i..].starts_with(second_list) {
            return Comparison::Superlist;
        }
    }

    for i in 0..second_list.len() {
        if second_list[i..].starts_with(first_list) {
            return Comparison::Sublist;
        }
    }

    Comparison::Unequal
}
