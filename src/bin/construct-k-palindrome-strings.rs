fn main() {
    println!("{}", Solution::can_construct("annabelle".to_string(), 2));
}

struct Solution;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut freq = vec![0; 26];
        let s = s.as_bytes();
        for &b in s {
            freq[(b - b'a') as usize] += 1;
        }
        let mut p = 0; // 奇数
        let mut q = 0; // 偶数
        for f in freq {
            p += f & 1;
            q += f >> 1;
        }
        k >= p && k <= p + 2 * q
    }
}
