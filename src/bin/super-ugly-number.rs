fn main() {
    println!(
        "{}",
        Solution::nth_super_ugly_number(5911, vec![2, 3, 5, 7])
    );
}
struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut ans: Vec<i64> = Vec::with_capacity(n);
        let mut heap = BinaryHeap::new();
        ans.push(1);
        for &p in primes.iter() {
            heap.push(Reverse((p as i64 * ans[0], p, 0)));
        }
        let Reverse((mut value, mut prime, mut idx)) = heap.pop().unwrap();
        for _ in 1..n {
            let last_value = value;
            ans.push(last_value);
            heap.push(Reverse((prime as i64 * ans[idx + 1], prime, idx + 1)));
            while !heap.is_empty() {
                let Reverse(t) = heap.pop().unwrap();
                (value, prime, idx) = t;
                if value != last_value {
                    break;
                }
                heap.push(Reverse((prime as i64 * ans[idx + 1], prime, idx + 1)));
            }
        }
        ans[n - 1] as i32
    }
}
