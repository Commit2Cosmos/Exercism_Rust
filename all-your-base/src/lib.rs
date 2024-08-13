#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    let mut invalid_dig: u32 = u32::default();

    //* convert to base 10 */
    let num_base_10:Option<Result<Vec<u32>, _>> = number.iter().rev()
    .all(|&x| {
        if x < from_base {
            true
        } else {
            invalid_dig = x as u32;
            false
        }
    })
    .then(|| number.iter().rev().enumerate()
        .map(|(i, &x)| {
            x as u32*from_base.pow(i as u32)
        })
        .sum::<u32>()
    )
    .and_then(|mut x| {
        let mut res: Vec<u32> = vec![];
        if x == 0 {
            res.push(0);
        }
        while x > 0 {
            res.push(x % to_base);
            x /= to_base;
        }
        Some(Ok(res.into_iter().rev().collect()))
    });

    if num_base_10.is_some() {
        num_base_10.unwrap()
    } else {
        Err(Error::InvalidDigit(invalid_dig))
    }


}
