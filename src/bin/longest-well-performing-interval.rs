fn main() {
    print!("{}", Solution::longest_wpi(vec![9, 9, 6, 0, 6, 6, 9]));
}

struct Solution;

impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        let mut ans = 0;
        let mut prefix_sum = 0;
        for (i, &v) in hours.iter().enumerate() {
            if v > 8 {
                prefix_sum += 1;
            } else {
                prefix_sum -= 1;
            }
            if prefix_sum > 0 {
                ans = std::cmp::max(ans, i + 1);
            }

            if let Some(&j) = map.get(&(prefix_sum - 1)) {
                ans = std::cmp::max(ans, i - j)
            }
            map.entry(prefix_sum).or_insert(i);
        }
        ans as i32
    }
}
