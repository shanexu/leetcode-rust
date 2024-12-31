fn main() {
    println!("{:?}", Solution::count_bits(10));
}

struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut ans = vec![0; n + 1];
        ans[0] = 0;
        for i in 1..=n {
            if i & 1 == 1 {
                ans[i] = ans[i - 1] + 1;
            } else {
                ans[i] = ans[i >> 1];
            }
        }
        ans
    }
}
