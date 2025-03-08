fn main() {
    assert_eq!(12, Solution::max_score(vec![1, 2, 3, 4, 5, 6, 1], 3));
}

struct Solution;

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        let n = card_points.len();
        let k = k as usize;
        for i in 0..k {
            sum += card_points[i];
        }
        if n == k {
            return sum;
        }
        let mut ans = sum;
        for i in 0..k {
            sum = sum - card_points[k - i - 1] + card_points[n - 1 - i];
            ans = ans.max(sum);
        }
        ans
    }
}
