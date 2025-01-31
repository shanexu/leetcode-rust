fn main() {
    assert_eq!(Solution::balanced_string_split("RLRRLLRLRL".to_string()), 4);
    assert_eq!(Solution::balanced_string_split("RLLLLRRRLR".to_string()), 3);
    assert_eq!(Solution::balanced_string_split("LLLLRRRR".to_string()), 1);
    assert_eq!(Solution::balanced_string_split("RLRRRLLRLL".to_string()), 2);
}

struct Solution;

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut ans = 0;
        for c in s.chars() {
            match c {
                'L' => l += 1,
                'R' => r += 1,
                _ => (),
            }
            if l == r {
                ans += 1;
            }
        }
        ans
    }
}
