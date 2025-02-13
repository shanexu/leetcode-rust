fn main() {
    assert_eq!(2, Solution::min_operations(vec![2, 11, 10, 1, 3], 10));
    assert_eq!(
        2,
        Solution::min_operations(vec![999999999, 999999999, 999999999], 1000000000)
    );
}

struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let mut heap = BinaryHeap::new();
        for num in nums {
            heap.push(Reverse(num as i64));
        }
        let mut ans = 0;
        while heap.peek().unwrap().0.lt(&k) {
            let min = heap.pop().unwrap().0;
            let max = heap.pop().unwrap().0;
            heap.push(Reverse(min * 2 + max));
            ans += 1;
        }
        ans
    }
}
