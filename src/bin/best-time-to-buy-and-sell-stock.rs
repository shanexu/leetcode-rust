fn main() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    assert_eq!(Solution::max_profit(vec![1, 2]), 1);
}

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n <= 1 {
            return 0;
        }
        let mut max_sum = 0;
        let mut current_sum = 0;
        for i in 0..(n - 1) {
            let x = prices[i + 1] - prices[i];
            if current_sum <= 0 {
                current_sum = x;
            } else {
                current_sum += x;
            }
            if current_sum > max_sum {
                max_sum = current_sum;
            }
        }
        max_sum
    }
}
