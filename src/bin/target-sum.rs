fn main() {
    println!(
        "{}",
        Solution3::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3)
    );
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

struct Solution2;

use std::collections::HashMap;
impl Solution2 {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        fn helper(
            nums: &Vec<i32>,
            i: usize,
            s: i32,
            target: i32,
            memo: &mut HashMap<(usize, i32), i32>,
        ) -> i32 {
            if target == s && i == nums.len() {
                return 1;
            }
            if i >= nums.len() {
                return 0;
            }
            if let Some(&c) = memo.get(&(i, s)) {
                return c;
            }
            let v = helper(nums, i + 1, s - nums[i], target, memo)
                + helper(nums, i + 1, s + nums[i], target, memo);
            memo.insert((i, s), v);
            v
        }
        let mut memo = HashMap::new();
        helper(&nums, 0, 0, target, &mut memo)
    }
}

struct Solution3;

impl Solution3 {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        fn helper(
            nums: &Vec<i32>,
            i: usize,
            s: i32,
            target: i32,
            total_sum: i32,
            memo: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if target == s && i == nums.len() {
                return 1;
            }
            if i >= nums.len() {
                return 0;
            }
            if memo[i][(s + total_sum) as usize] != -1 {
                return memo[i][(s + total_sum) as usize];
            }
            memo[i][(s + total_sum) as usize] =
                helper(nums, i + 1, s + nums[i], target, total_sum, memo)
                    + helper(nums, i + 1, s - nums[i], target, total_sum, memo);
            memo[i][(s + total_sum) as usize]
        }
        let total_sum = nums.iter().sum::<i32>();
        let mut memo = vec![vec![-1; (2 * total_sum + 1) as usize]; nums.len()];
        helper(&nums, 0, 0, target, total_sum, &mut memo)
    }
}
