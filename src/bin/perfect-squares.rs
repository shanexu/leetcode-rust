fn main() {
    println!("{}", Solution::num_squares(999));
    println!("{}", Solution2::num_squares(999));
    println!("{}", Solution3::num_squares(999));
    println!("{}", Solution4::num_squares(999));
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

struct Solution3;

impl Solution3 {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut arr = vec![];
        for i in 1.. {
            let s = i * i;
            if s > n {
                break;
            }
            arr.push(s);
        }
        let mut dp: Vec<i32> = vec![0; n + 1];
        for i in 1..=n {
            dp[i] = i as i32;
        }
        for i in 1..=n {
            for j in 0..arr.len() {
                if i >= arr[j] {
                    dp[i] = dp[i].min(dp[i - arr[j]] + 1);
                }
            }
        }
        dp[n]
    }
}

/// https://en.wikipedia.org/wiki/Lagrange%27s_four-square_theorem
struct Solution4;

impl Solution4 {
    pub fn num_squares(n: i32) -> i32 {
        fn is_square(n: i32) -> bool {
            let r = (n as f64).sqrt() as i32;
            r * r == n
        }

        if is_square(n) {
            return 1;
        }

        for i in 1.. {
            let s = i * i;
            if s > n {
                break;
            }
            if is_square(n - s) {
                return 2;
            }
        }

        let mut n = n;
        while n > 0 && n % 4 == 0 {
            n /= 4;
        }
        if n % 8 == 7 {
            return 4;
        }

        3
    }
}
