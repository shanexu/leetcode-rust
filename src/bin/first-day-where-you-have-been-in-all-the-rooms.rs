fn main() {
    println!(
        "{}",
        Solution::first_day_been_in_all_rooms(vec![
            0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
            13, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23, 24,
            24, 25, 25, 26, 26, 27, 27, 28, 28, 29, 29, 30, 30, 31, 31, 32, 32, 33, 33, 34, 34, 35,
            35, 36, 36, 37, 37, 38, 38, 39, 39, 40, 40, 41, 41, 42, 42, 43, 43, 44, 44, 45, 45, 46,
            46, 47, 47, 48,
        ])
    );
    println!("{}", Solution::first_day_been_in_all_rooms(vec![0, 0, 0]));
}

struct Solution;

impl Solution {
    pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
        let n = next_visit.len();
        let mut dp = vec![0; n];
        for i in 1..n {
            dp[i] = (dp[i - 1] + 2 + dp[i - 1] - dp[next_visit[i - 1] as usize]) % MOD;
            if dp[i] < 0 {
                dp[i] += MOD;
            }
        }
        dp[n - 1]
    }
}

const MOD: i32 = 1000000007;
