use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut res: BTreeMap<char, i32> = BTreeMap::new();
    h.iter().for_each(|(&i, x)| {
        x.iter().for_each(|&c| {
            res.insert(c.to_ascii_lowercase(), i);
        })
    });
    res
}
