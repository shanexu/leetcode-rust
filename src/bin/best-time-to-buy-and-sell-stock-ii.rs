fn main() {
    println!("{}", Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    println!("{}", Solution::max_profit(vec![1, 2, 3, 4, 5]));
    println!("{}", Solution::max_profit(vec![7, 6, 4, 3, 1]));
}

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
