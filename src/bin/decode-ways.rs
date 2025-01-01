fn main() {
    println!("{}", Solution::num_decodings("226".to_string()));
}

struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut res0 = if s[0] == b'0' { 0 } else { 1 };
        if s.len() == 1 {
            return res0;
        }
        if res0 == 0 {
            return 0;
        }
        let mut res1 = 0;
        let k = (s[0] - b'0') * 10 + s[1] - b'0';
        if k >= 10 && k <= 26 {
            res1 += res0;
        }
        if s[1] != b'0' {
            res1 += 1;
        }
        for i in 2..s.len() {
            let mut res = 0;
            let k = (s[i - 1] - b'0') * 10 + s[i] - b'0';
            if k >= 10 && k <= 26 {
                res += res0;
            }
            if s[i] != b'0' {
                res += res1;
            }
            std::mem::swap(&mut res1, &mut res);
            std::mem::swap(&mut res0, &mut res);
        }
        res1
    }
}
