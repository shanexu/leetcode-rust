fn main() {
    assert_eq!(6, Solution::subset_xor_sum(vec![1, 3]));
    assert_eq!(28, Solution::subset_xor_sum(vec![5, 1, 6]));
}

struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        help(&nums, 0, 0)
    }
}

fn help(nums: &Vec<i32>, idx: usize, curr: i32) -> i32 {
    if nums.len() == idx {
        return curr;
    }
    help(nums, idx + 1, curr ^ nums[idx]) + help(nums, idx + 1, curr)
}
