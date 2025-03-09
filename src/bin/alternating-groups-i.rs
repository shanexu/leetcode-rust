fn main() {
    assert_eq!(0, Solution::number_of_alternating_groups(vec![1, 1, 1]));
    assert_eq!(
        3,
        Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1])
    );
}

struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
        let n = colors.len();
        let mut ans = 0;
        let mut i = 0;
        let mut j = 1;
        while i < n {
            if colors[j % n] == colors[(j - 1) % n] {
                i = j;
                j = i + 1;
            } else {
                j += 1;
                if j == i + 3 {
                    ans += 1;
                    i += 1;
                    j -= 1;
                }
            }
        }
        ans
    }
}
