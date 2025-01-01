fn main() {}

struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        fn helper(
            i: usize,
            s: &str,
            dict: &[String],
            parts: &mut Vec<String>,
            ans: &mut Vec<String>,
        ) {
            if i == s.len() {
                ans.push(parts.join(" "));
            }
            let ts = &s[i..];
            for segment in dict {
                if ts.starts_with(segment) {
                    parts.push(segment.to_string());
                    helper(i + segment.len(), s, dict, parts, ans);
                    parts.pop();
                }
            }
        }
        let mut ans: Vec<String> = vec![];
        let mut parts: Vec<String> = vec![];
        helper(0, &s, &word_dict, &mut vec![], &mut ans);
        ans
    }
}
