use std::collections::{BTreeSet, HashMap, HashSet};

pub struct School {
    roster: HashMap<u32, BTreeSet<String>>,
    names: HashSet<String>
}

impl School {
    pub fn new() -> School {
        School { roster: HashMap::new(), names: HashSet::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self.names.contains(student) {
            return;
        }

        self.roster.entry(grade).and_modify(|x| {x.insert(student.to_string());}).or_insert(BTreeSet::from([student.to_string()]));
        self.names.insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.roster.keys().cloned().collect();
        grades.sort();
        grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.roster.get(&grade).cloned().or(Some(BTreeSet::new())).unwrap().into_iter().collect()
    }
}
