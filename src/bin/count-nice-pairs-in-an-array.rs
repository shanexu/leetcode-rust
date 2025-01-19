fn main() {
    println!("{:?}", Solution::count_nice_pairs(vec![42, 11, 1, 97]));
}

struct Solution;

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for num in nums {
            map.entry(num - rev(num))
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        let mut ans = 0u64;
        for (_, &v) in map.iter() {
            if v >= 2 {
                ans = (ans + v * (v - 1) / 2) % MOD;
            }
        }
        ans as i32
    }
}

#[inline(always)]
fn rev(mut num: i32) -> i32 {
    let mut ans = 0;
    while num > 0 {
        ans = ans * 10 + num % 10;
        num /= 10;
    }
    ans
}

const MOD: u64 = 1000_000_007;
