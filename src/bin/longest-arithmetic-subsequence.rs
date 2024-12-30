fn main() {
    println!("{}", Solution::longest_arith_seq_length(vec![1, 2, 3]));
    println!(
        "{}",
        Solution::longest_arith_seq_length(vec![1, 7, 10, 15, 27, 29])
    );
}

struct Solution;

use std::cmp::max;
use std::collections::HashMap;
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }
        let mut count = 0;
        let mut memo = HashMap::new();
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                let diff = nums[j] - nums[i];
                count = max(count, 2 + solve(i, diff, &nums, &mut memo));
            }
        }
        count
    }
}

fn solve(index: usize, diff: i32, nums: &Vec<i32>, memo: &mut HashMap<(usize, i32), i32>) -> i32 {
    if index == 0 {
        return 0;
    }

    if let Some(v) = memo.get(&(index, diff)) {
        return *v;
    }

    let mut count = 0;
    for j in (0..=(index - 1)).rev() {
        if nums[index] - nums[j] == diff {
            count = max(count, 1 + solve(j, diff, nums, memo));
        }
    }
    memo.insert((index, diff), count);
    count
}
