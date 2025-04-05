fn main() {
    assert_eq!(6, Solution::subset_xor_sum(vec![1, 3]));
    assert_eq!(28, Solution::subset_xor_sum(vec![5, 1, 6]));
}

struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut ans = 0;
        for num in nums {
            ans |= num;
        }
        ans << (n - 1)
    }
}
