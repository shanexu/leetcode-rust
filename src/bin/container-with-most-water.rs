fn main() {
    println!("{}", Solution::max_area(vec![1, 1]));
    println!("{}", Solution::max_area(vec![4, 3, 2, 1, 4]));
    println!("{}", Solution::max_area(vec![1, 2, 1]));
    println!("{}", Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
}

struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut m = 0i32;
        for i in 1..height.len() {
            let h = height[i];
            if h == 0 {
                continue;
            }
            let t = (i as i32) - m / h;
            if t <= 0 {
                continue;
            }
            for j in 0..(t as usize) {
                let s = min(height[j], h) * (i as i32 - j as i32);
                if s > m {
                    m = s;
                }
            }
        }
        m
    }
}

#[inline(always)]
fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}
