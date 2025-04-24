fn main() {
    println!("{:?}", Solution::count_largest_group(13));
}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut freq = HashMap::new();
        let mut max = 0;
        for i in 1..=n {
            let s = digit_sum(i);
            let e = freq.entry(s).or_insert(0);
            *e += 1;
            if max < *e {
                max = *e;
            }
        }
        let mut ans = 0;
        for (_, v) in freq {
            if v == max {
                ans += 1;
            }
        }
        ans
    }
}

#[inline]
fn digit_sum(mut n: i32) -> i32 {
    let mut s = 0;
    while n > 0 {
        s += n % 10;
        n /= 10;
    }
    s
}
