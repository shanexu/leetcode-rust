fn main() {
    println!(
        "{}",
        Solution::nth_super_ugly_number(5911, vec![2, 3, 5, 7])
    );
}

struct Solution;

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let n = n as usize;
        let k = primes.len();
        let mut indices = vec![0; k];
        let primes: Vec<u64> = primes.iter().map(|&x| x as u64).collect();
        let mut candidate: Vec<u64> = primes.clone();
        let mut min_value = u64::MAX;
        let mut min_indices: Vec<usize> = vec![0; k];
        let mut ans: Vec<u64> = Vec::with_capacity(n);
        ans.push(1);
        for _ in 1..n {
            for i in 0..k {
                if min_value > candidate[i] {
                    min_value = candidate[i];
                    min_indices.clear();
                    min_indices.push(i);
                } else if min_value == candidate[i] {
                    min_indices.push(i);
                }
            }
            ans.push(min_value);
            for &idx in min_indices.iter() {
                indices[idx] += 1;
                candidate[idx] = ans[indices[idx]] * primes[idx];
            }
            min_indices.clear();
            min_value = u64::MAX;
        }
        ans[n - 1] as i32
    }
}
