fn main() {
    println!("{:?}", Solution::most_competitive(vec![3, 5, 2, 6], 2));
    println!(
        "{:?}",
        Solution::most_competitive(vec![2, 4, 3, 3, 5, 4, 9, 6], 4)
    );
}

struct Solution;

impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut stack = Vec::with_capacity(k);
        let n = nums.len();
        for (i, &v) in nums.iter().enumerate() {
            while !stack.is_empty() && stack[stack.len() - 1] > v && (n - i > k - stack.len()) {
                stack.pop();
            }
            if stack.len() < k {
                stack.push(v);
            }
        }
        stack
    }
}
