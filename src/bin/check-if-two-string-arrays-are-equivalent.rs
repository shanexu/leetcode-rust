fn main() {
    println!(
        "{}",
        Solution::array_strings_are_equal(
            vec!["ab".to_string(), "c".to_string()],
            vec!["a".to_string(), "bc".to_string()]
        )
    );
}

struct Solution;

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let mut w1 = 0;
        let mut w2 = 0;
        let mut i1 = 0;
        let mut i2 = 0;
        let mut b1;
        let mut b2;
        loop {
            (w1, i1, b1) = next(w1, i1, &word1);
            (w2, i2, b2) = next(w2, i2, &word2);
            if b1 != b2 {
                return false;
            }
            if b1 == 0 && b2 == 0 {
                break;
            }
        }
        true
    }
}

#[inline]
fn next(mut w: usize, mut i: usize, word: &[String]) -> (usize, usize, u8) {
    if word.len() == w && i == 0 {
        return (w, i, 0);
    }
    let bs = word[w].as_bytes();
    let b = bs[i];
    if i + 1 == bs.len() {
        i = 0;
        w += 1;
    } else {
        i += 1;
    }
    (w, i, b)
}
