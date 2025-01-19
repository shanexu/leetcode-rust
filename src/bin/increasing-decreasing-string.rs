fn main() {}

struct Solution;

impl Solution {
    pub fn sort_string(s: String) -> String {
        let s = s.as_bytes();
        let n = s.len();
        let mut freq = vec![0; 26];
        for &b in s {
            freq[(b - b'a') as usize] += 1;
        }
        let mut ans = Vec::with_capacity(n);
        let mut rev = false;
        while ans.len() < n {
            if rev {
                for i in (0..26).rev() {
                    if freq[i] == 0 {
                        continue;
                    }
                    freq[i] -= 1;
                    ans.push(i as u8 + b'a');
                }
            } else {
                for i in 0..26 {
                    if freq[i] == 0 {
                        continue;
                    }
                    freq[i] -= 1;
                    ans.push(i as u8 + b'a');
                }
            }
            rev = !rev;
        }
        String::from_utf8(ans).unwrap()
    }
}