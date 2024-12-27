fn main() {

}

struct Solution;

/// 暴力算法
impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut final_prices = prices.clone();
        let n = prices.len();
        for i in 0..n {
            for j in (i + 1)..n {
                if prices[j] <= prices[i] {
                    final_prices[i] -= prices[j];
                    break;
                }
            }
        }
        final_prices
    }
}

/// 使用stack
struct Solution2;
impl Solution2 {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut final_prices = prices.clone();
        let mut stack = vec![];
        for i in 0..final_prices.len() {
            while !stack.is_empty() && prices[stack[stack.len()-1]] >= prices[i] {
                let p = stack.pop().unwrap();
                final_prices[p] -= prices[i];
            }
            stack.push(i);
        }
        final_prices
    }
}
