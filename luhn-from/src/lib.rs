pub struct Luhn {
    num: String
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let code = self.num.as_bytes()
        .iter()
        .filter(|&&x| x != b' ')
        .copied()
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
                    u - 9
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

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn {
            num: input.to_string()
        }
    }
}
