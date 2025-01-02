fn main() {
    println!("{}", Solution::count_digit_one(824883294));
    // for i in 0..1000 {
    //     println!("{}: {} {}", i, Solution2::count_digit_one(i), Solution::count_digit_one(i));
    // }
}

/// 非常天真的想法
struct Solution2;

impl Solution2 {
    pub fn count_digit_one(n: i32) -> i32 {
        let n = n as usize;
        let mut memo = vec![0; n + 1];
        for i in 1..=n {
            if i % 10 == 0 {
                memo[i] = memo[i / 10];
            } else if i % 10 == 1 {
                memo[i] = memo[i - 1] + 1;
            } else {
                memo[i] = memo[i / 10 * 10];
            }
        }
        let mut ans = 0i64;
        for &k in memo.iter() {
            ans += k as i64;
        }
        ans as i32
    }
}

/// 这个算法的思想就是，把1出现的位置分在个位，在十位，在百位...出现的次数分别计算。
/// 例如对于个位出现1的个数，应该是n以下，除以10余为1的数字的个数；对于十位出现1的个数，应该是n以下，除以100余为10的数字个数乘以10，依次类推
struct Solution;

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut f = 1;
        let mut ans = 0;
        while n >= f {
            let q = (n - f) / (f * 10);
            let r = (n - f) - (q * f * 10);
            let t = q * f + (if r >= f {
                f
            } else {
                r + 1
            });
            ans += t;
            f *= 10;
        }
        ans
    }
}
