fn main() {
    assert_eq!(
        2,
        Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1, 0, 1], 6)
    );
    assert_eq!(
        3,
        Solution::number_of_alternating_groups(vec![0, 1, 0, 1, 0], 3)
    );
    assert_eq!(
        0,
        Solution::number_of_alternating_groups(vec![1, 1, 0, 1], 4)
    );
    assert_eq!(
        4,
        Solution::number_of_alternating_groups(vec![0, 1, 0, 1], 3)
    );
    assert_eq!(
        1,
        Solution::number_of_alternating_groups(vec![0, 1, 0, 0], 3)
    );
}

struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let n = colors.len();
        let k = k as usize;
        let mut ans = 0;
        let mut i = 0;
        let mut j = 1;
        while i < n {
            if colors[(i + j) % n] == colors[(i + j - 1) % n] {
                i += j;
                j = 1;
            } else {
                j += 1;
                if j == k {
                    ans += 1;
                    i += 1;
                    j -= 1;
                }
            }
        }
        ans
    }
}
