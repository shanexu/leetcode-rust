fn main() {
    println!("{}", Solution::min_cost_climbing_stairs(vec![10, 15, 20]));
    println!(
        "{}",
        Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
    );
}

struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut ans = vec![0; n + 1];
        for i in 2..=n {
            ans[i] = std::cmp::min(ans[i - 1] + cost[i - 1], ans[i - 2] + cost[i - 2]);
        }
        ans[n]
    }
}
