fn main() {
    println!("{}", Solution::sum_subarray_mins(vec![3, 1, 2, 4]));
    println!("{}", Solution::sum_subarray_mins(vec![11, 81, 94, 43, 3]));
}

struct Solution;

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        const MODULO: i32 = 1_000_000_007;
        let mut sum = 0;
        let mut s = 0;
        let mut stack: Vec<usize> = vec![];
        for (i, &v) in arr.iter().enumerate() {
            while !stack.is_empty() && arr[stack[stack.len() - 1]] > v {
                let j = stack.pop().unwrap();
                let width = if stack.is_empty() {
                    j + 1
                } else {
                    j - stack[stack.len() - 1]
                };
                s -= (width as i32) * arr[j];
            }
            let width = if stack.is_empty() {
                i + 1
            } else {
                i - stack[stack.len() - 1]
            };
            s += (width as i32) * v;
            sum += s % MODULO;
            sum %= MODULO;
            stack.push(i);
        }
        sum
    }
}
