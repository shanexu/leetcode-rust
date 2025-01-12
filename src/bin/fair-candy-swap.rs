fn main() {
    println!("{:?}", Solution::fair_candy_swap(vec![1, 1], vec![2, 2]));
}

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let s1: i32 = alice_sizes.iter().sum();
        let s2: i32 = bob_sizes.iter().sum();
        let d = (s2 - s1) / 2;
        let set1: HashSet<i32> = alice_sizes.into_iter().collect();
        let set2: HashSet<i32> = bob_sizes.into_iter().collect();
        let mut ans = vec![0; 2];
        for &a in set1.iter() {
            let b = a + d;
            if set2.contains(&b) {
                ans[0] = a;
                ans[1] = b;
                break;
            }
        }
        ans
    }
}
