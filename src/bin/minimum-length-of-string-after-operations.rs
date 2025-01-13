fn main() {
    println!("{}", Solution::minimum_length("abaacbcbb".to_string()));
}

struct Solution;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let s = s.as_bytes();
        let mut freq = vec![0; 26];
        for &b in s {
            freq[(b - b'a') as usize] += 1;
        }
        let mut ans = 0;
        for f in freq {
            if f == 0 {
                continue;
            }
            if f % 2 == 0 {
                ans += 2;
            } else {
                ans += 1;
            }
        }
        ans
    }
}
