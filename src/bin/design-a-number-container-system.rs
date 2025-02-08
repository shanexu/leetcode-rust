fn main() {
    let mut nc = NumberContainers::new();
    nc.change(1, 10);
    println!("{}", nc.find(10));
    nc.change(1, 20);
    println!("{}", nc.find(10));
    println!("{}", nc.find(20));
}

use std::collections::{BTreeSet, HashMap};

struct NumberContainers {
    idx_to_num: HashMap<i32, i32>,
    num_to_idx: HashMap<i32, BTreeSet<i32>>,
}

impl NumberContainers {
    fn new() -> Self {
        Self {
            idx_to_num: HashMap::new(),
            num_to_idx: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        let itn = self.idx_to_num.entry(index).or_insert(0);
        if *itn != number {
            if let Some(nti) = self.num_to_idx.get_mut(itn) {
                nti.remove(&index);
                if nti.is_empty() {
                    self.num_to_idx.remove(itn);
                }
            }

            let nti = self.num_to_idx.entry(number).or_insert_with(BTreeSet::new);
            nti.insert(index);
            *itn = number;
        }
    }

    fn find(&self, number: i32) -> i32 {
        match self.num_to_idx.get(&number) {
            Some(nti) => *nti.first().unwrap(),
            None => -1,
        }
    }
}
