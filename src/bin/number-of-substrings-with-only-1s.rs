fn main() {
    println!("{}", Solution::num_sub("0110111".to_string()));
}

struct Solution;

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let mut ones: i64 = 0;
        let mut ans: i64 = 0;
        for c in s.chars() {
            if c == '1' {
                ones += 1;
            } else {
                if ones != 0 {
                    ans += ((1 + ones) * ones / 2) % MOD;
                    ans %= MOD;
                }
                ones = 0;
            }
        }
        if ones != 0 {
            ans += ((1 + ones) * ones / 2) % MOD;
            ans %= MOD;
        }
        ans as i32
    }
}

const MOD: i64 = 1_000_000_007;
