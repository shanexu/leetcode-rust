fn main() {
    assert_eq!(
        3,
        Solution::num_pairs_divisible_by60(vec![30, 20, 150, 100, 40])
    );
    assert_eq!(3, Solution::num_pairs_divisible_by60(vec![60, 60, 60]));
}

struct Solution;

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut freq = vec![0; 60];
        for t in time {
            freq[(t % 60) as usize] += 1;
        }
        let mut ans: i64 = 0;
        if freq[0] > 1 {
            ans += freq[0] * (freq[0] - 1) / 2;
        }
        if freq[30] > 1 {
            ans += freq[30] * (freq[30] - 1) / 2;
        }
        for i in 1..30 {
            ans += freq[i] * freq[60 - i];
        }
        ans as i32
    }
}
