fn main() {
    println!("{}", Solution::longest_palindrome("ccccddab".to_string()));
}

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut freq = vec![0; 52];
        let s = s.as_bytes();
        for &b in s {
            if b.is_ascii_uppercase() {
                freq[(b - b'A') as usize] += 1;
            } else {
                freq[(b - b'a' + 26) as usize] += 1;
            }
        }
        let mut ans = 0;
        for &c in freq.iter() {
            ans += c >> 1 << 1;
        }
        if freq.iter().any(|&c| c % 2 == 1) {
            ans += 1
        }
        ans
    }
}
