fn main() {}

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut ans = 0;
        for w in prices.windows(2) {
            if w[1] > w[0] {
                ans += w[1] - w[0]
            }
        }
        ans
    }
}
