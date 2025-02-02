fn main() {
    assert_eq!(Solution::judge_circle("UD".to_string()), true);
    assert_eq!(Solution::judge_circle("LL".to_string()), false);
}

struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut h = 0;
        let mut v = 0;
        for &d in moves.as_bytes() {
            match d {
                b'L' => h -= 1,
                b'R' => h += 1,
                b'U' => v += 1,
                b'D' => v -= 1,
                _ => {}
            }
        }
        h == 0 && v == 0
    }
}
