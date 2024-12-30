fn main() {
    println!("{}", Solution::count_good_strings(3, 3, 1, 1));
}

struct Solution;

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let high = high as usize;
        let low = low as usize;
        let zero = zero as usize;
        let one = one as usize;
        let mut memo = vec![0; high + 1];
        if zero <= high {
            memo[zero] += 1;
        }
        if one <= high {
            memo[one] += 1;
        }
        for i in 0..=high {
            if i + zero <= high {
                memo[i + zero] += memo[i];
                memo[i + zero] %= MOD
            }
            if i + one <= high {
                memo[i + one] += memo[i];
                memo[i + one] %= MOD
            }
        }
        let mut ans: i64 = 0;
        for i in low..=high {
            ans += memo[i];
            ans %= MOD;
        }
        ans as i32
    }
}

const MOD: i64 = 1_000_000_007;
