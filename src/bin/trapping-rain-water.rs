fn main() {
    println!(
        "{}",
        Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1])
    );
    println!("{}", Solution::trap(vec![4, 2, 0, 3, 2, 5]));
}

struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut left = vec![0; n];
        for i in 0..n {
            if i == 0 {
                left[i] = height[i];
                continue;
            }
            left[i] = max(left[i - 1], height[i]);
        }
        let mut right = vec![0; n];
        for i in (0..n).rev() {
            if i == n - 1 {
                right[i] = height[i];
                continue;
            }
            right[i] = max(right[i + 1], height[i]);
        }
        let mut s = 0;
        for i in 0..n {
            let h = height[i];
            let l = left[i];
            let r = right[i];
            if h < l && h < r {
                s += min(l, r) - h;
            }
        }
        s
    }
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}
