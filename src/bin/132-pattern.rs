fn main() {
    println!("{}", Solution::find132pattern(vec![1, 2, 3, 4]));
}

struct Solution;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut small = vec![i32::MIN; nums.len()];
        let mut min = nums[0];
        for (i, &v) in nums.iter().enumerate() {
            if min >= v {
                min = v;
            } else {
                small[i] = min;
            }
        }
        let mut stack = vec![];
        for i in (1..nums.len()).rev() {
            while !stack.is_empty() && stack[stack.len() - 1] <= small[i] {
                stack.pop();
            }
            if !stack.is_empty() && small[i] != i32::MIN && stack[stack.len() - 1] < nums[i] {
                return true;
            }
            stack.push(nums[i]);
        }
        false
    }
}
