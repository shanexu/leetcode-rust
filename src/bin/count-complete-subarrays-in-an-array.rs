fn main() {
    println!(
        "{}",
        Solution::count_complete_subarrays(vec![1, 3, 1, 2, 2])
    );
}

struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for &num in nums.iter() {
            set.insert(num);
        }
        let k = set.len();
        let mut left = 0;
        let mut right = 0;
        let mut freq = HashMap::new();
        let n = nums.len();
        let mut ans = 0;
        while right < n {
            let entry = freq.entry(nums[right]).or_insert(0);
            *entry += 1;
            right += 1;
            while freq.len() == k {
                ans += n - right + 1;
                let entry = freq.get_mut(&nums[left]).unwrap();
                *entry -= 1;
                if *entry == 0 {
                    freq.remove(&nums[left]);
                }
                left += 1;
            }
        }
        ans as _
    }
}
