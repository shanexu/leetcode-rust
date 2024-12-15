fn main() {
    println!(
        "{}",
        Solution::max_average_ratio(vec![vec![1, 2], vec![3, 5], vec![2, 2]], 2)
    );
    println!(
        "{}",
        Solution::max_average_ratio(vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]], 4)
    );
}

struct Solution;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let num = classes.len() as f64;
        let mut heap = BinaryHeap::new();
        let mut ratio_sum = 0.0;
        for v in classes {
            let p = v[0] as f64;
            let q = v[1] as f64;
            let dr = (q - p) / q / (q + 1.0);
            let r = p / q;
            ratio_sum += r;
            heap.push(Record(dr, p, q))
        }
        for _ in 0..extra_students {
            let Record(dr, p, q) = heap.pop().unwrap();
            let p = p + 1.0;
            let q = q + 1.0;
            ratio_sum += dr;
            let dr = (q - p) / q / (q + 1.0);
            heap.push(Record(dr, p, q))
        }
        ratio_sum / num
    }
}

struct Record(f64, f64, f64);

impl Eq for Record {}

impl PartialEq<Self> for Record {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl PartialOrd<Self> for Record {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 < other.0 {
            Some(Ordering::Less)
        } else if self.0 == other.0 {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 < other.0 {
            Ordering::Less
        } else if self.0 == other.0 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}
