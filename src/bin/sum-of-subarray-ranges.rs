fn main() {
    println!("{}", Solution::sub_array_ranges(vec![4, -2, -3, 4, 1]));
}

struct Solution;

impl Solution {
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let mut stack: Vec<usize> = vec![];
        let mut min_sum: i64 = 0;
        let mut s: i64 = 0;
        let mut max_sum: i64 = 0;
        for (i, &v) in nums.iter().enumerate() {
            while !stack.is_empty() && v < nums[stack[stack.len() - 1]] {
                let j = stack.pop().unwrap();
                let prev: i64 = if stack.is_empty() {
                    -1
                } else {
                    stack[stack.len() - 1] as i64
                };
                s -= nums[j] as i64 * (j as i64 - prev);
            }
            let prev: i64 = if stack.is_empty() {
                -1
            } else {
                stack[stack.len() - 1] as i64
            };
            s += nums[i] as i64 * (i as i64 - prev);
            min_sum += s;
            stack.push(i);
        }
        stack.clear();
        s = 0;
        for (i, &v) in nums.iter().enumerate() {
            while !stack.is_empty() && v > nums[stack[stack.len() - 1]] {
                let j = stack.pop().unwrap();
                let prev: i64 = if stack.is_empty() {
                    -1
                } else {
                    stack[stack.len() - 1] as i64
                };
                s -= nums[j] as i64 * (j as i64 - prev);
            }
            let prev: i64 = if stack.is_empty() {
                -1
            } else {
                stack[stack.len() - 1] as i64
            };
            s += nums[i] as i64 * (i as i64 - prev);
            max_sum += s;
            stack.push(i);
        }

        max_sum - min_sum
    }
}
