fn main() {
    println!(
        "{}",
        Solution::nth_super_ugly_number(5911, vec![2,3,5,7])
    );
}

struct Solution;

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut ans: Vec<i64> = Vec::with_capacity(n);
        ans.push(1);
        let mut set = std::collections::BTreeSet::new();
        for &p in primes.iter() {
            set.insert((p as i64 * ans[0], p, 0));
        }
        let (mut value, mut prime, mut idx) = set.pop_first().unwrap();
        for i in 1..n {
            let last_value = value;
            ans.push(last_value);
            set.insert((prime as i64 * ans[idx + 1], prime, idx + 1));
            while !set.is_empty() {
                (value, prime, idx) = set.pop_first().unwrap();
                if value != last_value {
                    break;
                }
                set.insert((prime as i64 * ans[idx + 1], prime, idx + 1));
            }
        }
        ans[n - 1] as i32
    }
}
