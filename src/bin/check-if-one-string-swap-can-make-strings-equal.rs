fn main() {
    assert_eq!(Solution::are_almost_equal("bank".to_string(), "kanb".to_string()), true);
    assert_eq!(Solution::are_almost_equal("attack".to_string(), "defend".to_string()), false);
    assert_eq!(Solution::are_almost_equal("kelb".to_string(), "kelb".to_string()), true);
    assert_eq!(Solution::are_almost_equal("abcd".to_string(), "dcba".to_string()), false);
    assert_eq!(Solution::are_almost_equal("aa".to_string(), "bb".to_string()), false);
    assert_eq!(Solution::are_almost_equal("ab".to_string(), "ba".to_string()), true);
    assert_eq!(Solution::are_almost_equal("ab".to_string(), "ab".to_string()), true);
    assert_eq!(Solution::are_almost_equal("ab".to_string(), "ac".to_string()), false);
    assert_eq!(Solution::are_almost_equal("ab".to_string(), "ca".to_string()), false);
    assert_eq!(Solution::are_almost_equal("ab".to_string(), "cb".to_string()), false);
    assert_eq!(Solution::are_almost_equal("ab".to_string(), "bc".to_string()), false);
    assert_eq!(Solution::are_almost_equal("ab".to_string(), "cd".to_string()), false);
    assert_eq!(Solution::are_almost_equal("ab".to_string(), "dc".to_string()), false);
    assert_eq!(Solution::are_almost_equal("ab".to_string(), "cd".to_string()), false);
    assert_eq!(Solution::are_almost_equal("ab".to_string(), "dc".to_string()), false);
    assert_eq!(Solution::are_almost_equal("ab".to_string(), "cd".to_string()), false);
    assert_eq!(Solution::are_almost_equal("ab".to_string(), "dc".to_string()), false);
    assert_eq!(Solution::are_almost_equal("ab".to_string(), "cd".to_string()), false);
    assert_eq!(Solution::are_almost_equal("ab".to_string(), "dc".to_string()), false);
    assert_eq!(Solution::are_almost_equal("ab".to_string(), "cd".to_string()), false);
    assert_eq!(Solution::are_almost_equal("ab".to_string(), "dc".to_string()), false);
}

struct Solution;

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut diff = vec![];
        let n = s1.len();
        for i in 0..n {
            let b1 = s1[i];
            let b2 = s2[i];
            if b1 != b2 {
                if diff.len() == 4 {
                    return false;
                }
                diff.push(b1);
                diff.push(b2);
            }
        }
        if diff.len() == 0 {
            return true;
        }
        if diff.len() == 2 {
            return false;
        }
        diff[0] == diff[3] && diff[2] == diff[1]
    }
}
