fn main() {
    println!("{}", Solution::count_primes(2));
}

struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        let mut primes = vec![true; n];
        let mut p = 2;
        while p * p < n {
            if primes[p] {
                for i in (p * p..n).step_by(p) {
                    primes[i] = false;
                }
            }
            p += 1;
        }
        primes.iter().skip(2).filter(|x| **x).count() as i32
    }
}
