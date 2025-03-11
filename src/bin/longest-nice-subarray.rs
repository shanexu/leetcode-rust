fn main() {
    assert_eq!(3, Solution::longest_nice_subarray(vec![1, 3, 8, 48, 10]));
    assert_eq!(1, Solution::longest_nice_subarray(vec![3, 1, 5, 11, 13]));
}

struct Solution;

impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut sum = nums[0];
        let mut ans = 1;
        for i in 1..nums.len() {
            let num = nums[i];
            while sum & num != 0 {
                sum -= nums[left];
                left += 1;
            }
            sum += num;
            ans = ans.max(i - left + 1);
        }
        ans as i32
    }
}
