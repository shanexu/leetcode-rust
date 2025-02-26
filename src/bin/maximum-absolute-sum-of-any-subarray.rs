fn main() {
    assert_eq!(5, Solution::max_absolute_sum(vec![1, -3, 2, 3, -4]));
}

struct Solution;

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut sum = nums[0];
        let mut min = 0.min(sum);
        let mut max = 0.max(sum);
        let mut max_abs = min.abs().max(max.abs());
        let mut ans = sum.abs();
        let n = nums.len();
        for i in 1..n {
            let num = nums[i];
            sum += num;
            if sum < 0 {
                ans = ans.max((sum - max).abs());
            } else if sum > 0 {
                ans = ans.max((sum - min).abs());
            } else {
                ans = ans.max(max_abs);
            }
            min = min.min(sum);
            max = max.max(sum);
            max_abs = max_abs.max(min.abs()).max(max_abs);
        }
        ans
    }
}
