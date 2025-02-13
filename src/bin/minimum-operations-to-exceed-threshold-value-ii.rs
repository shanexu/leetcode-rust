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
        let mut heap = nums
            .into_iter()
            .map(|x| Reverse(x as i64))
            .collect::<BinaryHeap<Reverse<i64>>>();
        let mut ans = 0;
        loop {
            let min = heap.pop().unwrap().0;
            if min >= k {
                break;
            }
            let max = heap.pop().unwrap().0;
            heap.push(Reverse(min * 2 + max));
            ans += 1;
        }
        ans
    }
}
