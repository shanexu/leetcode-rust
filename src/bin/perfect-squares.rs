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
            let mut ans = i32::MAX;
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

struct Solution2;

impl Solution2 {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp: Vec<i32> = vec![0; n + 1];
        dp[0] = 0;
        dp[1] = 1;
        for i in 2..=n {
            dp[i] = i as i32;
            let mut x = 1;
            while x * x <= i {
                dp[i] = dp[i].min(dp[i - x * x] + 1);
                x += 1;
            }
        }
        dp[n]
    }
}
