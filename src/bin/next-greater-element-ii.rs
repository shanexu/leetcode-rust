fn main() {
    println!("{:?}", Solution::next_greater_elements(vec![1, 2, 3, 4, 3]));
}

struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![-1; nums.len()];
        let mut stack: Vec<usize> = vec![];
        for i in 0..nums.len() {
            while !stack.is_empty() && nums[stack[stack.len() - 1]] < nums[i] {
                let j = stack.pop().unwrap();
                result[j] = nums[i];
            }
            stack.push(i);
        }
        let max = nums[stack[0]];
        for i in 0..nums.len() {
            while !stack.is_empty() && nums[stack[stack.len() - 1]] < nums[i] {
                let j = stack.pop().unwrap();
                result[j] = nums[i];
            }
            if nums[stack[stack.len() - 1]] == max {
                break;
            }
        }
        result
    }
}
