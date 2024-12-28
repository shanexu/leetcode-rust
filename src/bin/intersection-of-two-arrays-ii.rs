fn main() {}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut num_counts = HashMap::new();
        let mut result = Vec::new();

        for num in nums1 {
            *num_counts.entry(num).or_insert(0) += 1;
        }

        for num in nums2 {
            if let Some(count) = num_counts.get_mut(&num) {
                result.push(num);
                *count -= 1;
                if *count == 0 {
                    num_counts.remove(&num);
                }
            }
        }

        result
    }
}
