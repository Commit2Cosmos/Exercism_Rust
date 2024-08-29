
pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {

    if input.is_empty() {
        return Some(Vec::new());
    }

    let mut used = vec![false; input.len()];
    let mut result = vec![input[0]];
    used[0] = true;
    
    if find_next(input[0], &mut used, &input, &mut result).is_some() {
        return Some(result);
    }
    
    None
}

fn find_next(
    curr: (u8, u8),
    used: &mut Vec<bool>,
    input: &[(u8, u8)],
    res: &mut Vec<(u8, u8)>
) -> Option<Vec<(u8, u8)>> {
    if res.len() == input.len() {
        if res[0].0 == res[res.len()-1].1 {
            return Some(res.to_vec());
        } else {
            return None;
        }
    }

    for i in 0..input.len() {
        if !used[i] {
            let mut dom = input[i];
            println!("{curr:?} : {dom:?}");
            if curr.1 == dom.0 {
                println!("passed normal");
                used[i] = true;
                res.push(dom);
                if let Some(r) = find_next(dom, used, input, res) {
                    // println!("{r:?}");
                    return Some(r);
                };
                used[i] = false;
                res.pop();
            } else if curr.1 == dom.1 {
                println!("passed flipped");
                used[i] = true;
                dom = (dom.1, dom.0);
                res.push(dom);
                if let Some(r) = find_next(dom, used, input, res) {
                    // println!("{r:?}");
                    return Some(r);
                };
                used[i] = false;
                res.pop();
            }
        }
    }

    None
}