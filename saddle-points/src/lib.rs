
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    
    let mut res: Vec<(usize, usize)> = vec![];

    for (i, row) in input.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            let jth_col: Vec<u64> = input.iter().map(|x| x[j]).collect();
            //* if value is the largest in its row and smallest in its column */
            if row.iter().all(|x| v >= x) && jth_col.iter().all(|x| v <= x) {
                res.push((i, j));
            }
        }
    }

    res
}
