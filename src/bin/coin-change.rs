fn main() {
    println!("{}", Solution::coin_change(vec![1, 2, 5], 11));
}

struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut ans = vec![i32::MAX; amount + 1];
        ans[0] = 0;
        for i in 0..amount {
            if ans[i] == i32::MAX {
                continue;
            }
            for &coin in &coins {
                if i + coin as usize <= amount {
                    ans[i + coin as usize] = ans[i + coin as usize].min(ans[i] + 1);
                }
            }
        }
        if ans[amount] == i32::MAX {
            -1
        } else {
            ans[amount]
        }
    }
}
