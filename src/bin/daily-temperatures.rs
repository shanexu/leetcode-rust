fn main() {
    println!(
        "{:?}",
        Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73])
    );
}

struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<usize> = vec![];
        let mut ans: Vec<i32> = vec![0; temperatures.len()];

        for i in 0..temperatures.len() {
            while !stack.is_empty() && temperatures[stack[stack.len() - 1]] < temperatures[i] {
                let idx = stack.pop().unwrap();
                ans[idx] = (i - idx) as i32
            }
            stack.push(i);
        }
        ans
    }
}
