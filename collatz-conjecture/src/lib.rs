use std::iter::successors;

pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    Some(
        successors(Some(n), |&x| {
        if x == 1 {
            None
        } else if x % 2 == 0 {
            Some(x / 2)
        } else {
            Some(x * 3 + 1)
        }
        })
        .count() as u64 - 1
    )
}