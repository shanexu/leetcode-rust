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
        let mut heap: BinaryHeap<Reverse<(i64, i64, usize)>> = BinaryHeap::with_capacity(primes.len());
        ans.push(1);
        for &p in primes.iter() {
            heap.push(Reverse((p as i64 * ans[0], p as i64, 0)));
        }
        for _ in 1..n {
            let mut last_value = 0;
            loop {
                let mut peek = heap.peek_mut().unwrap();
                if last_value != 0 {
                    if peek.0 .0 != last_value {
                        break;
                    }
                } else {
                    last_value = peek.0 .0;
                    ans.push(last_value);
                }
                *peek = Reverse((
                    peek.0 .1 * ans[peek.0 .2 + 1],
                    peek.0 .1,
                    peek.0 .2 + 1,
                ));
            }
        }
        ans[n - 1] as i32
    }
}
