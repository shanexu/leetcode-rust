fn main() {
    println!(
        "{:?}",
        Solution::max_sum_of_three_subarrays(vec![4, 5, 10, 6, 11, 17, 4, 11, 1, 3], 1)
    );
}

struct Solution;

impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut sum1 = 0;
        let mut sum2 = 0;
        let mut sum3 = 0;
        let mut max_idx1: usize = 0;
        let mut max_idx12: (usize, usize) = (0, 0);
        let mut max_idx123: (usize, usize, usize) = (0, 0, 0);
        let mut max_sum1 = 0;
        let mut max_sum12 = 0;
        let mut max_sum = 0;
        for i in 2 * k..nums.len() {
            sum1 += nums[i - 2 * k];
            sum2 += nums[i - k];
            sum3 += nums[i];
            if i >= 3 * k - 1 {
                if sum1 > max_sum1 {
                    max_sum1 = sum1;
                    max_idx1 = i + 1 - 3 * k;
                }
                if max_sum1 + sum2 > max_sum12 {
                    max_sum12 = max_sum1 + sum2;
                    max_idx12 = (max_idx1, i + 1 - 2 * k);
                }
                if max_sum12 + sum3 > max_sum {
                    max_sum = max_sum12 + sum3;
                    max_idx123 = (max_idx12.0, max_idx12.1, i + 1 - k);
                }
                sum1 -= nums[i + 1 - 3 * k];
                sum2 -= nums[i + 1 - 2 * k];
                sum3 -= nums[i + 1 - k];
            }
        }
        vec![
            max_idx123.0 as i32,
            max_idx123.1 as i32,
            max_idx123.2 as i32,
        ]
    }
}
