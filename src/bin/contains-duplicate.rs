fn main() {}

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::with_capacity(nums.len());
        for n in nums {
            if !set.insert(n) {
                return true;
            }
        }
        false
    }
}
