use leetcode_rust::vec_vec_i32;

fn main() {
    Solution::is_zero_array(vec![1, 0, 1], vec_vec_i32![[0, 2]]);
}

struct Solution;

impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let n = nums.len();
        let mut diff = vec![0; n + 1];
        for q in queries {
            diff[q[0] as usize] -= 1;
            diff[q[1] as usize + 1] += 1;
        }
        let mut sum = 0;
        for i in 0..n {
            sum += diff[i];
            if nums[i] + sum > 0 {
                return false;
            }
        }
        true
    }
}
