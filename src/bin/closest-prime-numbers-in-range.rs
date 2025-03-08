fn main() {
    println!("{:?}", Solution::closest_primes(19, 31));
}

struct Solution;

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut ans = vec![-1, -1];
        let left = left as usize;
        let right = right as usize;
        let mut primes = vec![true; right + 1];
        primes[1] = false;
        let mut p = 2;
        while p * p <= right {
            if primes[p] {
                for i in ((p * p)..=right).step_by(p) {
                    primes[i] = false;
                }
            }
            p += 1;
        }
        let mut prev = 1;
        let mut min = right - left + 1;
        for i in left..=right {
            if primes[i] {
                if prev != 1 && i - prev < min {
                    min = i - prev;
                    ans[0] = prev as i32;
                    ans[1] = i as i32;
                    if min == 2 {
                        return ans;
                    }
                }
                prev = i;
            }
        }
        ans
    }
}
