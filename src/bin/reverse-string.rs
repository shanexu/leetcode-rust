fn main() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    Solution::reverse_string(&mut s);
    assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    Solution::reverse_string(&mut s);
    assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
}

struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let n = s.len();
        for i in 0..(n / 2) {
            s.swap(i, n - 1 - i);
        }
    }
}
