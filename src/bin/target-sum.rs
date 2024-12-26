fn main() {
    println!("{}", Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3));
}

struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        fn helper(nums: &[i32], target: i32) -> i32 {
            if nums.len() == 0 {
                if target == 0 {
                    return 1;
                }
                return 0;
            }
            helper(&nums[1..], target + nums[0]) + helper(&nums[1..], target - nums[0])
        }

        helper(&nums[..], target)
    }
}
