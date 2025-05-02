pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let code = self.to_string();
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
}
