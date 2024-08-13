/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.as_bytes()
        .iter()
        .filter(|&&x| x != b' ')
        .map(|&x| x)
        .collect::<Vec<u8>>();

    if code.iter().any(|x| !x.is_ascii_digit()) || code.len() < 2 {
        return false;
    }

    code.iter().rev()
        .enumerate()
        .map(|(i, x)| {
            if i % 2 == 1 {
                let u = (x - b'0') as usize * 2;
                if u > 9 {
                    (u+1) % 10
                } else {
                    u
                }
            } else {
                (x - b'0') as usize
            }
        })
        .sum::<usize>() % 10 == 0

}