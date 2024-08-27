use std::thread;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {

    if input.is_empty() { return HashMap::new(); }    
    
    thread::scope(|s| {
        let mut handles = Vec::with_capacity(worker_count);

        for lines in input.chunks(input.len() / worker_count + 1) {
            handles.push(s.spawn(|| {
                let mut h: HashMap<char, usize> = HashMap::new();
                lines.iter().for_each(|c| {
                    c.chars().into_iter()
                    .filter(|x| { x.is_alphabetic() })
                    .for_each(|x| {
                        *h.entry(x.to_ascii_lowercase()).or_default() += 1;
                    });
                });
                h
            }))
        }

        let mut map = handles.pop().unwrap().join().unwrap();
        
        handles.into_iter().for_each(|h| {
            h.join().unwrap().iter().for_each(|(key, val)| {
                *map.entry(*key).or_default() += val;
            })
        });

        map
    })
}
