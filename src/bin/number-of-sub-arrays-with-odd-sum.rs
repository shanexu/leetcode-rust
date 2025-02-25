fn main() {
    assert_eq!(4, Solution::num_of_subarrays(vec![1, 3, 5]));
}

struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut prefix_sum = arr[0];
        let mut odd_sum = arr[0] & 1;
        let mut ans = odd_sum;
        for i in 1..n {
            prefix_sum += arr[i];
            if prefix_sum & 1 == 1 {
                ans += i as i32 - odd_sum + 1;
                odd_sum += 1;
            } else {
                ans += odd_sum;
            }
            ans %= MOD;
        }
        ans
    }
}

const MOD: i32 = 1_000_000_007;
