fn main() {
    println!(
        "{}",
        Solution::min_increment_for_unique(vec![3, 2, 1, 2, 1, 7])
    );
}

struct Solution;

impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut ans = 0;
        let mut prev = nums[0];
        for i in 1..nums.len() {
            if nums[i] <= prev {
                let diff = prev - nums[i] + 1;
                nums[i] += diff;
                ans += diff;
            }
            prev = nums[i];
        }
        ans
    }
}
