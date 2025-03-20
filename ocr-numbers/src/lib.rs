// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    //* Check input dims
    let rows: Vec<&str> = input.split('\n').collect();

    if rows.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(rows.len()));
    } else if rows.iter().any(|&r| r.len() % 3 != 0) {
        return Err(Error::InvalidColumnCount(rows[0].len()));
    }

    let letters = vec![
        (" _ | ||_|   ", "0"), ("     |  |   ", "1"), (" _  _||_    ", "2"),
        (" _  _| _|   ", "3"), ("   |_|  |   ", "4"), (" _ |_  _|   ", "5"),
        (" _ |_ |_|   ", "6"), (" _   |  |   ", "7"), (" _ |_||_|   ", "8"), (" _ |_| _|   ", "9")
    ].iter().cloned().collect::<HashMap<&str, &str>>();


    let mut res = String::new();

    for row in rows.chunks_exact(4) {
        for col in (0..rows[0].len()).step_by(3) {
            let letter: String = row.iter().flat_map(|&s| s[col..col+3].chars()).collect();
            res += letters.get(letter.as_str()).unwrap_or(&"?");
        }
        res += ",";
    }
    res.pop();

    Ok(res)
}
