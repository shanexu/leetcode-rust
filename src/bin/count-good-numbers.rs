fn main() {
    assert_eq!(5, Solution::count_good_numbers(1));
    assert_eq!(400, Solution::count_good_numbers(4));
    assert_eq!(564908303, Solution::count_good_numbers(50));
}

struct Solution;

impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        let ans = fast_pow(5, (n + 1) / 2) * fast_pow(4, n / 2);
        (ans % MOD) as _
    }
}

const MOD: i64 = 1000000007;

fn fast_pow(mut base: i64, mut n: i64) -> i64 {
    let mut res = 1;
    while n > 0 {
        if n % 2 == 1 {
            res *= base;
            res %= MOD;
        }
        base *= base;
        base %= MOD;
        n /= 2;
    }
    res % MOD
}
