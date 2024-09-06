use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|x| {
        let mut c = *x;
        let mut bytes = VecDeque::new();
        let mut is_last = true;

        while c != 0 || is_last {
            let byte = (c & 0x7F) as u8;
            c >>= 7;
            
            if is_last {
                bytes.push_front(byte);
                is_last = false;
            } else {
                bytes.push_front(byte | 0x80);
            }
        }

        bytes
    })
    .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut curr = 0u32;
    let mut res = vec![];
    let mut cont_num = false;

    for x in bytes.iter() {

        curr = (curr << 7) | (x & 0x7F) as u32;

        if (x & 0x80) != 0 {
            cont_num = true;
        } else {
            res.push(curr);
            cont_num = false;
            curr = 0;
        }
    }

    match cont_num {
        true => Err(Error::IncompleteNumber),
        false => Ok(res),
    }
}
