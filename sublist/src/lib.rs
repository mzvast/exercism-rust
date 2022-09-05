#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        Comparison::Equal
    } else if contains(_first_list, _second_list) {
        Comparison::Superlist
    } else if contains(_second_list, _first_list) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}

fn contains<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() < b.len() {
        return false;
    } 

    if a.starts_with(b) {
        return true;
    }

    contains(&a[1..], b)
}
