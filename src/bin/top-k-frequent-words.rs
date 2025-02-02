fn main() {
    let binding = "i".to_string();
    let a = (Reverse(10), &binding);
    let binding = "love".to_string();
    let b = (Reverse(10), &binding);
    println!("{:?}", a.cmp(&b));
    let mut h = BinaryHeap::new();
    h.push(a);
    h.push(b);
    println!("{:?}", h.peek());

    println!(
        "{:?}",
        Solution::top_k_frequent(
            vec![
                "i".to_string(),
                "love".to_string(),
                "leetcode".to_string(),
                "i".to_string(),
                "love".to_string(),
                "coding".to_string()
            ],
            3
        )
    );
}

struct Solution;

use std::cmp::Ordering::Greater;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut freq = HashMap::new();
        for w in words.iter() {
            freq.entry(w).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut heap = std::collections::BinaryHeap::new();
        let k = k as usize;
        for (i, (&w, &c)) in freq.iter().enumerate() {
            if i < k {
                heap.push((Reverse(c), w));
            } else {
                let mut peek = heap.peek_mut().unwrap();
                let value = (Reverse(c), w);
                if peek.cmp(&value) == Greater {
                    *peek = value;
                }
            }
        }
        let mut ans: Vec<String> = Vec::with_capacity(k);
        while let Some((_, w)) = heap.pop() {
            ans.push(w.clone());
        }
        ans.reverse();
        ans
    }
}
