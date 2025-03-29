fn main() {
    assert_eq!(
        4788,
        Solution::maximum_score(vec![19, 12, 14, 6, 10, 18], 3)
    );
}

struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let max_element = *nums.iter().max().unwrap();
        let primes = get_primes(max_element);
        let n = nums.len();
        let mut scores = vec![0; n];
        for (i, &num) in nums.iter().enumerate() {
            scores[i] = get_score(num, &primes);
        }
        let mut next = vec![n as i32; n];
        let mut prev = vec![-1i32; n];
        let mut stack: Vec<usize> = vec![];
        for i in 0..n {
            while !stack.is_empty() && scores[stack[stack.len() - 1]] < scores[i] {
                let top = stack.pop().unwrap();
                next[top] = i as i32;
            }
            if !stack.is_empty() {
                prev[i] = stack[stack.len() - 1] as i32;
            }
            stack.push(i);
        }
        let mut subarrays = vec![0i64; n];
        for i in 0..n {
            subarrays[i] = (next[i] as i64 - i as i64) * (i as i64 - prev[i] as i64);
        }
        let mut heap = BinaryHeap::new();
        for (i, &num) in nums.iter().enumerate() {
            heap.push((num, Reverse(i)));
        }
        let mut ans = 1i64;
        let mut k = k as i64;
        while k > 0 {
            let (num, Reverse(idx)) = heap.pop().unwrap();
            let ops = k.min(subarrays[idx]);
            ans = (ans * power(num as i64, ops)) % MOD;
            k -= ops;
        }
        ans as _
    }
}

const MOD: i64 = 1_000_000_007;

#[inline]
fn get_primes(limit: i32) -> Vec<i32> {
    let n = limit as usize;
    let mut primes = vec![true; n + 1];
    let mut p = 2;
    while p * p <= n {
        if primes[p] {
            for i in (p * p..=n).step_by(p) {
                primes[i] = false;
            }
        }
        p += 1;
    }
    let mut ans = vec![];
    for i in 2..=n {
        if primes[i] {
            ans.push(i as i32);
        }
    }
    ans
}

#[inline]
fn get_score(mut num: i32, primes: &Vec<i32>) -> i32 {
    let mut s = 0;
    for &p in primes {
        if p * p > num {
            break;
        }
        if num % p != 0 {
            continue;
        }
        s += 1;
        while num % p == 0 {
            num /= p;
        }
    }
    if num > 1 {
        s += 1;
    }
    s
}

#[inline]
fn power(mut base: i64, mut exp: i64) -> i64 {
    let mut res = 1i64;
    while exp > 0 {
        if exp % 2 == 1 {
            res = (res * base) % MOD;
        }

        base = (base * base) % MOD;
        exp /= 2;
    }
    res
}
