fn main() {
    println!("{}", Solution::clear_digits("cb34".to_string()));
}

struct Solution;

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let s = s.as_bytes();
        let mut ans: Vec<u8> = vec![];
        for &b in s {
            if b >= b'0' && b <= b'9' {
                ans.pop();
            } else {
                ans.push(b);
            }
        }
        String::from_utf8(ans).unwrap()
    }
}
