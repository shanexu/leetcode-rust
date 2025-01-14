fn main() {
    println!("{:?}", Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2));
}

struct Solution;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for num in nums {
            map.entry(num).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut heap = BinaryHeap::new();
        let k = k as usize;
        for (i, (&num, &count)) in map.iter().enumerate() {
            if i < k {
                heap.push(Reverse((count, num)));
            } else {
                let mut peek = heap.peek_mut().unwrap();
                if (*peek).0 .0 < count {
                    *peek = Reverse((count, num));
                }
            }
        }
        heap.into_iter().map(|Reverse((_, x))| x).collect()
    }
}
