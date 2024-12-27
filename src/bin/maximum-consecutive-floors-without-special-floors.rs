fn main() {
    println!("{}", Solution::max_consecutive(2, 9, vec![4, 6]));
}

struct Solution;

impl Solution {
    pub fn max_consecutive(bottom: i32, top: i32, mut special: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut ps = bottom - 1;
        special.sort();
        for s in special {
            ans = std::cmp::max(ans, s - ps - 1);
            ps = s;
        }
        ans = std::cmp::max(ans, top - ps);
        ans
    }
}
