fn main() {
    println!(
        "{}",
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
    );
}

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut current_sum = nums[0];
        for i in 1..nums.len() {
            let x = nums[i];
            if current_sum <= 0 {
                current_sum = x;
            } else {
                current_sum += x;
            }
            if current_sum > max_sum {
                max_sum = current_sum;
            }
        }
        max_sum
    }
}
