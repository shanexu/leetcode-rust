fn main() {}

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