fn main() {
    assert_eq!(Solution::is_ugly(6), true);
    assert_eq!(Solution::is_ugly(8), true);
    assert_eq!(Solution::is_ugly(14), false);
    assert_eq!(Solution::is_ugly(1), true);
    assert_eq!(Solution::is_ugly(0), false);
    assert_eq!(Solution::is_ugly(-6), false);
}

struct Solution;

impl Solution {
    fn is_ugly(mut n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        for &factor in [2, 3, 5].iter() {
            while n % factor == 0 {
                n /= factor;
            }
        }
        n == 1
    }
}
