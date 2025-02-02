fn main() {
    assert_eq!(
        Solution::can_construct("a".to_string(), "b".to_string()),
        false
    );
    assert_eq!(
        Solution::can_construct("aa".to_string(), "ab".to_string()),
        false
    );
    assert_eq!(
        Solution::can_construct("aa".to_string(), "aab".to_string()),
        true
    );
}

struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        fn freq(s: &[u8]) -> [i32; 26] {
            let mut f = [0; 26];
            for &b in s {
                f[(b - b'a') as usize] += 1;
            }
            f
        }

        let ransom_note = ransom_note.as_bytes();
        let magazine = magazine.as_bytes();
        let f1 = freq(ransom_note);
        let f2 = freq(magazine);
        for i in 0..26 {
            if f1[i] > f2[i] {
                return false;
            }
        }
        true
    }
}
