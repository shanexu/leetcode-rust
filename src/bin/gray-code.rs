fn main() {
    println!("{:?}", Solution::gray_code(1));
    println!("{:?}", Solution::gray_code(2));
    println!("{:?}", Solution::gray_code(3));
}

struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut ans = vec![0; 1 << n];
        let mut p: usize = 1;
        for _ in 0..n {
            for i in 0..p {
                ans[2 * p - i - 1] = ans[i] + p as i32;
            }
            p <<= 1;
        }
        ans
    }
}
