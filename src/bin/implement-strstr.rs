struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle, None) {
            Some(i) => i as i32,
            None => -1
        }
    }
}
