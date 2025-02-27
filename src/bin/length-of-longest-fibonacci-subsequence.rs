fn main() {
    assert_eq!(
        5,
        Solution::len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8])
    );
    assert_eq!(0, Solution::len_longest_fib_subseq(vec![1, 3, 5]));
}

struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let set: HashSet<i32> = arr.iter().cloned().collect();
        let mut dp = HashMap::new();
        let mut ans = 0;
        for i in 0..n {
            for j in 1..i {
                let w = arr[i];
                let v = arr[j];
                let u = w - v;
                if u < v && set.contains(&u) {
                    let len = *dp.get(&(u, v)).unwrap_or(&2) + 1;
                    dp.insert((v, w), len);
                    ans = ans.max(len);
                }
            }
        }
        ans
    }
}
