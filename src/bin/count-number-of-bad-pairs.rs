fn main() {
    println!("{}", Solution::count_bad_pairs(vec![4, 1, 3, 3]));
}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let n = nums.len() as i64;
        let mut map: HashMap<i32, i64> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            *map.entry(num - i as i32).or_insert(0) += 1
        }
        let mut ans = n * (n - 1) / 2;
        for (_, &c) in map.iter() {
            if c > 1 {
                ans -= c * (c - 1) / 2;
            }
        }
        ans
    }
}
