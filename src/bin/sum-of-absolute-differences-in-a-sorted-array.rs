fn main() {
    println!(
        "{:?}",
        Solution::get_sum_absolute_differences(vec![2, 3, 5])
    );
    println!(
        "{:?}",
        Solution::get_sum_absolute_differences(vec![1, 4, 6, 8, 10])
    );
}

struct Solution;

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![0; n];
        let mut sum = 0;
        for i in 0..n {
            sum += nums[i];
            ans[i] = sum;
        }
        for i in (0..n).rev() {
            let left = if i == 0 {
                0
            } else {
                nums[i] * i as i32 - ans[i - 1]
            };
            let right = sum - ans[i] - nums[i] * (n - i - 1) as i32;
            ans[i] = left + right;
        }
        ans
    }
}
