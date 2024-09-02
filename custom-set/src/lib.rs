use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    data: Vec<T>
}

impl<T: Clone + PartialEq + Ord + Display> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut d = input.to_vec();
        d.sort();
        d.dedup();
        Self {
            data: d
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.binary_search(element).is_ok()
    }

    pub fn add(&mut self, element: T) {
        if let Err(idx) = self.data.binary_search(&element) {
            self.data.insert(idx, element)
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        if other.data.len() < self.data.len() {
            return false;
        }

        let mut pointer = 0;

        for x in other.data.iter() {
            if let Some(k) = self.data.get(pointer) {
                if x == k {
                    pointer += 1;
                }
            }
        }

        pointer == self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        let mut pointer = 0;

        for x in other.data.iter() {
            while let Some(k) = self.data.get(pointer) {
                match k.cmp(x) {
                    std::cmp::Ordering::Less => pointer += 1,
                    std::cmp::Ordering::Equal => return false,
                    std::cmp::Ordering::Greater => break,
                }
            }
        }

        true
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {

        let mut pointer_og = 0;
        let mut pointer_oth = 0;

        let mut data = Vec::new();

        while pointer_og < self.data.len() && pointer_oth < other.data.len() {

            match self.data[pointer_og].cmp(&other.data[pointer_oth]) {
                std::cmp::Ordering::Less => pointer_og += 1,
                std::cmp::Ordering::Greater => pointer_oth += 1,
                std::cmp::Ordering::Equal => {
                    data.push(self.data[pointer_og].clone());
                    pointer_og += 1;
                    pointer_oth += 1;
                },
            }
        }

        Self { data }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {

        let mut pointer_oth = 0;

        let data = self.data.iter().filter_map(|x| {
            if pointer_oth < other.data.len() {
                match x.cmp(&other.data[pointer_oth]) {
                    std::cmp::Ordering::Less => {
                        Some(x.clone())
                    },
                    std::cmp::Ordering::Greater => {
                        pointer_oth += 1;
                        None
                    },
                    std::cmp::Ordering::Equal => {
                        pointer_oth += 1;
                        None
                    },
                }
            } else {
                Some(x.clone())
            }
        })
        .collect();

        Self { data }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {

        let mut pointer_og = 0;
        let mut pointer_oth = 0;

        let mut data = Vec::new();

        while pointer_og < self.data.len() && pointer_oth < other.data.len() {
            match self.data[pointer_og].cmp(&other.data[pointer_oth]) {
                std::cmp::Ordering::Less => {
                    data.push(self.data[pointer_og].clone());
                    pointer_og += 1;
                },
                std::cmp::Ordering::Greater => {
                    data.push(other.data[pointer_oth].clone());
                    pointer_oth += 1;
                },
                std::cmp::Ordering::Equal => {
                    data.push(self.data[pointer_og].clone());
                    pointer_og += 1;
                    pointer_oth += 1;
                },
            }
        }

        if pointer_og == self.data.len() {
            for x in other.data[pointer_oth..].iter() {
                data.push(x.clone());
            }
        } else if pointer_oth == other.data.len() {
            for x in self.data[pointer_og..].iter() {
                data.push(x.clone());
            }
        }

        Self { data }
    }
}
