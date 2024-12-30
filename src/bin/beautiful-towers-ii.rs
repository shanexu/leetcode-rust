fn main() {}

/// 完全和 beautiful-towers-i 一样的代码
struct Solution;

impl Solution {
    pub fn maximum_sum_of_heights(mut heights: Vec<i32>) -> i64 {
        fn helper(heights: &Vec<i32>) -> Vec<i64> {
            let mut stack: Vec<usize> = vec![];
            let mut s: i64 = 0;
            let mut ss = vec![];
            for (i, &v) in heights.iter().enumerate() {
                while !stack.is_empty() && heights[stack[stack.len() - 1]] > v {
                    let j = stack.pop().unwrap();
                    let width = if stack.is_empty() {
                        j + 1
                    } else {
                        j - stack[stack.len() - 1]
                    };
                    s -= (width as i64) * (heights[j] as i64);
                }
                let width = if stack.is_empty() {
                    i + 1
                } else {
                    i - stack[stack.len() - 1]
                };
                s += (width as i64) * (v as i64);
                ss.push(s);
                stack.push(i);
            }
            ss
        }
        let n = heights.len();
        // 从左往右算一遍
        let s1 = helper(&heights);
        heights.reverse();
        // 从右往左算一遍
        let s2 = helper(&heights);
        let mut m = i64::MIN;
        for i in 0..n {
            // 这里注意要去掉重复计算的peak的高度
            let s = s1[i] + s2[n - i - 1] - heights[n - i - 1] as i64;
            m = std::cmp::max(m, s);
        }
        m
    }
}
