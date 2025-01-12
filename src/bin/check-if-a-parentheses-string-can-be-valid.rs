fn main() {
    println!(
        "{}",
        Solution::can_be_valid(
            "())()))()(()(((())(()()))))((((()())(())".to_string(),
            "1011101100010001001011000000110010100101".to_string()
        )
    );
}

struct Solution;

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let s = s.as_bytes();
        let locked = locked.as_bytes();
        let n = s.len();
        if n & 1 == 1 {
            return false;
        }
        let mut open_or_unlocked = 0usize;
        for i in 0..n {
            let b = s[i];
            let l = locked[i];
            if b == b'(' || l == b'0' {
                open_or_unlocked += 1;
            } else if open_or_unlocked > 0 {
                open_or_unlocked -= 1;
            } else {
                return false;
            }
        }
        open_or_unlocked = 0;
        for i in (0..n).rev() {
            let b = s[i];
            let l = locked[i];
            if b == b')' || l == b'0' {
                open_or_unlocked += 1;
            } else if open_or_unlocked > 0 {
                open_or_unlocked -= 1;
            } else {
                return false;
            }
        }
        true
    }
}
