fn main() {
    println!("{}", Solution::num_squares(999));
}

struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        fn helper(n: usize, memo: &mut Vec<i32>) -> i32 {
            if n == 0 {
                return 0;
            }
            if memo[n] != -1 {
                return memo[n];
            }
            let root = (n as f64).sqrt() as usize;
            let mut ans = i32::max_value();
            for i in (1..=root).rev() {
                ans = ans.min(helper(n - i * i, memo));
            }
            ans += 1;
            memo[n] = ans;
            ans
        }
        let n = n as usize;
        helper(n as usize, &mut vec![-1; n + 1])
    }
}
