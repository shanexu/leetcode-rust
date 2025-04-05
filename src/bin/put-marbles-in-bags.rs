fn main() {
    println!("{}", Solution::put_marbles(vec![1, 3, 5, 1], 2));
}

struct Solution;

impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let n = weights.len();
        let mut pair_sums = weights
            .windows(2)
            .into_iter()
            .map(|w| w[0] + w[1])
            .collect::<Vec<i32>>();
        pair_sums.sort_unstable();
        let k = k as usize;
        let mut min_sum = 0i64;
        let mut max_sum = 0i64;
        for i in 0..k - 1 {
            min_sum += pair_sums[i] as i64;
            max_sum += pair_sums[n - i - 2] as i64;
        }
        max_sum - min_sum
    }
}
