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
        let mut heap = nums
            .into_iter()
            .map(|x| Reverse(x))
            .collect::<BinaryHeap<Reverse<i32>>>();
        let mut ans = 0;
        while let Some(Reverse(min)) = heap.pop() {
            if min >= k {
                break;
            }
            let max = heap.pop().unwrap().0;
            heap.push(Reverse(
                min.checked_mul(2)
                    .and_then(|x| max.checked_add(x))
                    .unwrap_or(i32::MAX),
            ));
            ans += 1;
        }
        ans
    }
}
