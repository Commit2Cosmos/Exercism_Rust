/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() { return None; }

    let res = s1.as_bytes().into_iter().zip(s2.as_bytes().into_iter()).map(|(x, y)| {
        if x == y { 0 } else { 1 }
    })
    .sum();

    Some(res)
}
