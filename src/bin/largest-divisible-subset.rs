fn main() {
    println!("{:?}", Solution::largest_divisible_subset(vec![1, 2, 3]));
}

struct Solution;

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let n = nums.len();
        let mut dp = vec![1usize; n];
        let mut max_idx = 0;
        for i in 0..n {
            for j in 0..i {
                if nums[i] % nums[j] == 0 {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            if dp[i] > dp[max_idx] {
                max_idx = i;
            }
        }
        let mut max_size = dp[max_idx];
        let mut ans = vec![0; max_size];
        let mut ans_idx = ans.len() - 1;
        let mut curr_idx = max_idx;
        loop {
            if nums[max_idx] % nums[curr_idx] == 0 && dp[curr_idx] == max_size {
                ans[ans_idx] = nums[curr_idx];
                max_size -= 1;
                if max_size == 0 {
                    break;
                }
                ans_idx -= 1;
                max_idx = curr_idx;
            }
            curr_idx -= 1;
        }
        ans
    }
}
