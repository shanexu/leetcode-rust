fn main() {
    println!(
        "{}",
        Solution::total_steps(vec![7, 14, 15, 14, 13, 1, 2, 3, 4, 13, 2, 6, 13])
    );
}

struct Solution;

impl Solution {
    pub fn total_steps(nums: Vec<i32>) -> i32 {
        let mut stack: Vec<(usize, usize)> = vec![];
        let mut ans = 0;
        for (i, &v) in nums.iter().enumerate().rev() {
            let mut steps: usize = 0;
            while !stack.is_empty() && v > nums[stack[stack.len() - 1].0] {
                let (_, t) = stack.pop().unwrap();
                steps = std::cmp::max(steps + 1, t);
            }
            ans = std::cmp::max(ans, steps);
            stack.push((i, steps))
        }
        ans as i32
    }
}
