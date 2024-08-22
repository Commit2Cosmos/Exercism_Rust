#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist_old<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.is_empty() {
        if _second_list.is_empty() {
            return Comparison::Equal;
        }
        return Comparison::Sublist;
    } else if _second_list.is_empty() {
        return Comparison::Superlist;
    }

    let (larger, smaller) = (_first_list.len() > _second_list.len()).then_some((_first_list, _second_list)).or(Some((_second_list, _first_list))).unwrap();
    
    larger.windows(smaller.len()).any(|window| window == smaller).then(|| {
        if _first_list.len() == _second_list.len() {
            Comparison::Equal
        } else if std::ptr::eq(larger, _first_list) {
            Comparison::Superlist
        } else {
            Comparison::Sublist
        }
    })
    .or(Some(Comparison::Unequal))
    .unwrap()
}


pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    let sublist = first.is_empty() || second.windows(first.len()).any(|x| x == first);
    let superlist = second.is_empty() || first.windows(second.len()).any(|x| x == second);

    match (sublist, superlist) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}