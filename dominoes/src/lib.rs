use std::collections::HashMap;


pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.is_empty() { return Some(Vec::new()); }

    //* number on domino -> indices of dominoes which contain them */
    let mut num2stone = HashMap::new();

    for (i, &(a, b)) in input.iter().enumerate() {
        num2stone.entry(a).or_insert(Vec::new()).push(i);
        num2stone.entry(b).or_insert(Vec::new()).push(i);
    }

    //* Not all dominoes can be joined if there's uneven number of any of the numbers */
    if num2stone.values().any(|x| x.len() % 2 != 0) { return None; }

    let (mut curr_path, mut final_path) = (Vec::new(), Vec::new());
    
    //* Pick arbitrary starting point (a random number present on some dominoes)
    //* Treated as an end of a domino
    let mut point: u8 = *num2stone.keys().next().unwrap();

    loop {
        match num2stone.get_mut(&point).unwrap().pop() {
            //* If there are still dominoes that can be connected */
            Some(idx) => {
                let (start, end) = input[idx];
                //* Check if last of point matches the first of current, otherwise flip, as it's guaranteed that the domino contains that number */
                let (start, end) = if start == point { (start, end) } else { (end, start) };

                point = end;

                //* Add the domino to the current path */
                curr_path.push((start, end));
                
                //* Remove the used number on the used domino from the leftover dominoes */
                num2stone.get_mut(&end).unwrap().retain(|x| *x != idx);
            },
            //* If there are no domminoes the current one can be connected to 
            //* Backtract until an alternative route can be found
            None => {
                if let Some((start, end)) = curr_path.pop() {
                    final_path.push((end, start));
                    point = start;
                } else { 
                    //* if there're no other points we could have used to reach this domino -> chain cannot be formed -> length of final_path will be smaller than the number of dominoes
                    break;
                }
            },
        }
    }

    // println!("{:?}", final_path);

    if final_path.len() != input.len() {
        None
    } else {
        Some(final_path)
    }
}