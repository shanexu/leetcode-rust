fn main() {}

struct Solution;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let s = s.as_bytes();
        let n = s.len();
        let mut count = 0;
        for i in 0..n / 2 {
            match s[i] {
                b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U' => count += 1,
                _ => (),
            }
        }
        for i in n / 2..n {
            match s[i] {
                b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U' => count -= 1,
                _ => (),
            }
        }
        count == 0
    }
}
