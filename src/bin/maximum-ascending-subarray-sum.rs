fn main() {
    println!(
        "{}",
        Solution::max_ascending_sum(vec![10, 20, 30, 5, 10, 50])
    );
}

struct Solution;

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut s = nums[0];
        let mut prev = nums[0];
        for &num in nums.iter().skip(1) {
            s = if num <= prev { num } else { s + num };
            ans = ans.max(s);
            prev = num;
        }
        ans
    }
}
