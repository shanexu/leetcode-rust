fn main() {
    assert_eq!(
        Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]),
        10,
        "case1"
    );
    assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4, "case2");
}

// 暴力
struct Solution0 {}

impl Solution0 {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut m = 0;
        for i in 0..heights.len() {
            let mut h = heights[i];
            m = std::cmp::max(h, m);
            for j in (i + 1)..heights.len() {
                h = std::cmp::min(heights[j], h);
                m = std::cmp::max(h * (j - i + 1) as i32, m);
            }
        }
        m
    }
}

// stack
// https://www.geeksforgeeks.org/largest-rectangular-area-in-a-histogram-using-stack/#divide-and-conquer-approach-on-log-n
struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = vec![];
        let mut res = 0;
        for i in 0..heights.len() {
            while !stack.is_empty() && heights[i] <= heights[*stack.last().unwrap()] {
                let idx = stack.pop().unwrap();
                let width = if stack.is_empty() {
                    i as i32
                } else {
                    (i - stack.last().unwrap() - 1) as i32
                };
                res = max(res, heights[idx] * width)
            }
            stack.push(i);
        }

        while !stack.is_empty() {
            let idx = stack.pop().unwrap();
            let width = if stack.is_empty() {
                heights.len() as i32
            } else {
                (heights.len() - stack.last().unwrap() - 1) as i32
            };
            res = max(res, heights[idx] * width);
        }

        res
    }
}

#[inline]
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}
