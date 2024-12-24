fn main() {
    println!("{}", Solution::mct_from_leaf_values(vec![4, 2, 3, 1]));
}

struct Solution;

impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        let mut stack = vec![i32::MAX];
        let mut result = 0;

        for &v in arr.iter() {
            while stack[stack.len() - 1] <= v {
                let u = stack.pop().unwrap();
                result += u * std::cmp::min(stack[stack.len() - 1], v);
            }
            stack.push(v);
        }

        for i in 2..stack.len() {
            result += stack[i] * stack[i - 1];
        }

        result
    }
}
