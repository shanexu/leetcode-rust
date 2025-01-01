fn main() {}

/// Moore’s Voting Algorithm
struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = i32::MIN;
        let mut count = 0;
        for num in nums {
            if count == 0 {
                candidate = num;
                count += 1;
            } else if candidate == num {
                count += 1;
            } else {
                count -= 1;
            }
        }
        candidate
    }
}
