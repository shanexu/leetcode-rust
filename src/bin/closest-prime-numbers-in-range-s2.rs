fn main() {
    println!("{:?}", Solution::closest_primes(19, 31));
}

struct Solution;

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut ans = vec![-1, -1];
        let left = left as usize;
        let right = right as usize;
        let mut primes = new_bit_map(right + 1);
        unset_bit(1, &mut primes);
        let mut p = 2;
        while p * p <= right {
            if get_bit(p, &mut primes) {
                for i in (p * p..=right).step_by(p) {
                    unset_bit(i, &mut primes);
                }
            }
            p += 1;
        }
        let mut prev = 1;
        let mut min = right - left + 1;
        for i in left..=right {
            if get_bit(i, &mut primes) {
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

#[inline]
fn new_bit_map(total: usize) -> Vec<u64> {
    let len = (total + 63) >> 6;
    vec![u64::MAX; len]
}

#[inline]
fn get_bit(n: usize, bit_map: &mut Vec<u64>) -> bool {
    let i = n >> 6;
    let k = n & 63;
    ((bit_map[i] >> k) & 1) != 0
}

#[inline]
fn unset_bit(n: usize, bit_map: &mut Vec<u64>) {
    let i = n >> 6;
    let k = n & 63;
    bit_map[i] &= !(1 << k);
}
