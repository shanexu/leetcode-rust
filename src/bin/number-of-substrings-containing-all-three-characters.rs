fn main() {
    assert_eq!(10, Solution::number_of_substrings("abcabc".to_string()));
    assert_eq!(3, Solution::number_of_substrings("aaacb".to_string()));
    assert_eq!(1, Solution::number_of_substrings("abc".to_string()));
}

struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        const MASK: usize = 0b111;
        let mut flags: usize = 0;
        let mut freq = [0; 3];
        let mut ans = 0;
        let mut left = 0;
        let s = s.as_bytes();
        for &b in s {
            let i = (b - b'a') as usize;
            freq[i] += 1;
            flags |= 1 << i;
            while flags == MASK {
                let b = s[left];
                let i = (b - b'a') as usize;
                freq[i] -= 1;
                if freq[i] == 0 {
                    flags -= 1 << i;
                }
                left += 1;
            }
            ans += left as i32;
        }
        ans
    }
}
