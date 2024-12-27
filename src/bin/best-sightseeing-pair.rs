fn main() {
    println!("{}", Solution::max_score_sightseeing_pair(vec![1, 3, 5]));
}

struct Solution;

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut ans = values[0] + values[1] + 0 - 1;
        let mut max = std::cmp::max(values[0] + 0, values[1] + 1);
        for (i, &v) in values.iter().enumerate().skip(2) {
            ans = std::cmp::max(ans, max + v - i as i32);
            max = std::cmp::max(max, v + i as i32);
        }
        ans
    }
}
