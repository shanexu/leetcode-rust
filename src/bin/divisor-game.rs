fn main() {
    println!("{}", Solution::divisor_game(6));
}

struct Solution;

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        fn helper(n: usize, memo: &mut Vec<i8>) -> bool {
            if n == 1 {
                return false;
            }
            if memo[n] != -1 {
                return memo[n] == 1;
            }
            for i in 1..=(n as f64).sqrt() as usize {
                if n % i == 0 {
                    if !helper(n - i, memo) {
                        memo[n] = 1;
                        return true;
                    }
                }
            }
            memo[n] = 0;
            false
        }
        let n = n as usize;
        let mut memo = vec![-1; n + 1];
        helper(n, &mut memo)
    }
}
